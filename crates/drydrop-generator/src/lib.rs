use drydrop_core::error::DryDropResult;
use drydrop_registry::{LocalRegistry, RemoteRegistry};
use crate::context::GenerationContext;
use crate::output::GenerationOutput;

pub mod output;
pub mod context;
pub mod merge;
pub mod planner;
pub mod resolver;
pub mod pipeline;

pub struct Generator<L: LocalRegistry, R: RemoteRegistry> {
    pub local_registry: L,
    pub remote_registry: R,
}

impl <L: LocalRegistry, R: RemoteRegistry> Generator<L, R> {
    pub fn new(local_registry: L, remote_registry: R) -> Self {
        Self {
            local_registry,
            remote_registry
        }
    }
    // Todo
    pub fn generate(&self, context: GenerationContext) -> DryDropResult<GenerationOutput> {
        let project_dir = context.project.output_dir.value().to_owned();
        Ok(GenerationOutput::new(project_dir))
    }
}
