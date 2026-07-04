use drydrop_core::error::DryDropResult;
use drydrop_core::project::Project;
use drydrop_engine::Engine;

pub struct GenerationContext {
    pub project: Project,
    pub engine: Engine,
}

impl GenerationContext {
    pub fn new(
        project_name: impl Into<String>,
        output_dir: impl Into<String>,
        engine: Engine,
    ) -> DryDropResult<Self> {
        Ok(Self {
            project: Project::new(project_name, output_dir)?,
            engine,
        })
    }
}
