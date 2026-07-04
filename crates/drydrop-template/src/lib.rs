use drydrop_core::module::Module;
use drydrop_core::variable::Variables;
use drydrop_vfs::tree::VfsTree;

pub struct Template {
    pub module: Module,
    pub root: VfsTree,
    pub variables: Vec<Variables>,
}

impl Template {
    pub fn new() -> Self {
        Self {
            module: Module::new(),
            root: VfsTree::new(),
            variables: Vec::new(),
        }
    }
    // Todo
    pub fn walk_dir() -> Self {
        Self {
            module: Module::new(),
            root: VfsTree::new(),
            variables: Vec::new(),
        }
    }
    pub fn get_by_name(name: impl Into<String>) -> Self {
        let template = Template {
            module: Module::new(),
            root: VfsTree::new(),
            variables: Vec::new(),
        };
        template
    }
}
