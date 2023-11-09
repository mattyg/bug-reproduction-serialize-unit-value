use hdk::prelude::*;

/// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn get_u32(_: ()) -> ExternResult<u32> {
    Ok(1)
}

#[hdk_extern]
pub fn get_usize(_: ()) -> ExternResult<usize> {
    Ok(1)
}
