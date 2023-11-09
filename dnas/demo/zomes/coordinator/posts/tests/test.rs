use hdk::prelude::*;
use holochain::{conductor::{config::ConductorConfig, api::error::ConductorApiError}, sweettest::*, prelude::DnaFile};
use holochain::test_utils::consistency_60s;
  
#[tokio::test(flavor = "multi_thread")]
async fn can_serialize_unit_value() {
    // Use prebuilt dna file
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../../workdir/demo.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    // Set up conductors
    let mut conductor: SweetConductor= SweetConductor::from_config(ConductorConfig::default()).await;
    let app = conductor.setup_app("demo", &[dna]).await.unwrap();
    let (alice,) = app.into_tuple();

    // Alice gets a u32 response
    let value: u32 = conductor
        .call(&alice.zome("posts"), "get_u32", ())
        .await;
    
    assert_eq!(value, 1);

    // Alice gets a usize response
    let value: usize = conductor
        .call(&alice.zome("posts"), "get_usize", ())
        .await;
    
    assert_eq!(value, 1);
}
