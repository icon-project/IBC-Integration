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

impl<'a> CwIbcCoreContext<'a> {
    fn get_route(
        &self,
        module_id: &ibc::core::ics26_routing::context::ModuleId,
    ) -> Option<&dyn ibc::core::ics26_routing::context::Module> {
        todo!()
    }

    fn get_route_mut(
        &mut self,
        module_id: &ibc::core::ics26_routing::context::ModuleId,
    ) -> Option<&mut dyn ibc::core::ics26_routing::context::Module> {
        todo!()
    }

    fn has_route(&self, module_id: &ibc::core::ics26_routing::context::ModuleId) -> bool {
        todo!()
    }

    fn lookup_module_by_port(
        &self,
        port_id: &ibc::core::ics24_host::identifier::PortId,
    ) -> Option<ibc::core::ics26_routing::context::ModuleId> {
        todo!()
    }
}
