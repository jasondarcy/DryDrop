use crate::module::module_category::ModuleCategory;
use crate::module::module_id::ModuleId;
use crate::module::module_name::ModuleName;
use crate::module::module_description::ModuleDescription;
use crate::module::module_dependency::ModuleDependency;
use crate::module::module_version::ModuleVersion;

pub mod module_id;
pub mod module_name;
pub mod module_description;
pub mod module_dependency;
pub mod module_version;
pub mod module_category;

pub struct Module {
    pub id: ModuleId,
    pub name: ModuleName,
    pub description: ModuleDescription,
    pub version: ModuleVersion,
    pub category: ModuleCategory,
    pub dependencies: Vec<ModuleDependency>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            id: ModuleId::new(),
            name: ModuleName::new(),
            description: ModuleDescription::new(),
            version: ModuleVersion::new(),
            category: ModuleCategory::None,
            dependencies: Vec::new(),
        }
    }
}
