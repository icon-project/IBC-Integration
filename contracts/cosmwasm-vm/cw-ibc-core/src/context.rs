use super::*;
pub struct CwIbcCoreConext<'a> {
    cw_ibc_store: CwIbcStore<'a>,
    cw_ibc_router: CwIbcRouter<'a>,
}
