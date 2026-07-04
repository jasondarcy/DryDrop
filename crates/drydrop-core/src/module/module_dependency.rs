use crate::module::module_id::ModuleId;

pub struct ModuleDependency {
    pub required: Vec<ModuleId>,
    pub conflicts: Vec<ModuleId>,
}

impl ModuleDependency {
    pub fn new() -> Self {
        Self {
            required: Vec::new(),
            conflicts: Vec::new(),
        }
    }
}
