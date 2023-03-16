use super::*;
pub struct CwIbcRouter<'a>(Map<'a, ModuleId, Arc<dyn Module>>);

impl<'a> Default for CwIbcRouter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcRouter<'a> {
    pub fn new() -> Self {
        Self(Map::new(StorageKey::Router.as_str()))
    }
}
