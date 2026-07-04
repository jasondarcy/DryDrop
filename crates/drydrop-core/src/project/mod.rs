pub mod project_name;
pub mod project_output_dir;

use crate::error::DryDropResult;
use crate::module::module_id::ModuleId;
use crate::project::project_name::ProjectName;
use crate::project::project_output_dir::ProjectOutputDir;

pub struct Project {
    pub name: ProjectName,
    pub output_dir: ProjectOutputDir,
    pub modules: Vec<ModuleId>,
}

impl Project {
    pub fn new(project_name: impl Into<String>, output_dir: impl Into<String>) -> DryDropResult<Self> {
        Ok(
            Self {
                name: ProjectName::new(project_name)?,
                output_dir: ProjectOutputDir::new(output_dir)?,
                modules: Vec::new(),
            }
        )
    }
}
