use indexmap::IndexMap;
use drydrop_plugin::Plugin;

pub struct PluginRepository {
    pub plugins: IndexMap<String, Plugin>,
}

impl PluginRepository {
    pub fn new() -> Self {
        Self {
            plugins: IndexMap::new(),
        }
    }
}

impl Default for PluginRepository {
    fn default() -> Self {
        Self {
            plugins: IndexMap::new(),
        }
    }
}
