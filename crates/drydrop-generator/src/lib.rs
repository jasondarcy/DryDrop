use drydrop_registry::{LocalRegistry, RemoteRegistry};

pub mod input;
pub mod merge;
pub mod output;
pub mod pipeline;
pub mod planner;
pub mod resolver;

pub struct Generator<L: LocalRegistry, R: RemoteRegistry> {
    pub local_registry: L,
    pub remote_registry: R,
}

impl<L: LocalRegistry, R: RemoteRegistry> Generator<L, R> {
    pub fn new(local_registry: L, remote_registry: R) -> Self {
        Self {
            local_registry,
            remote_registry,
        }
    }
}
