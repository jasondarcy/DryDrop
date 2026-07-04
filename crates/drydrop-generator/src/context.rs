use drydrop_core::error::DryDropResult;
use drydrop_core::project::Project;

pub struct GenerationContext {
    pub project: Project,
}

impl GenerationContext {
    pub fn new(project_name: impl Into<String>, output_dir: impl Into<String>) -> DryDropResult<Self> {
        Ok(
            Self {
                project: Project::new(project_name, output_dir)?,
            }
        )
    }
}
