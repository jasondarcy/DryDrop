use async_trait::async_trait;

pub mod repository;

pub trait LocalRegistry {
    fn get_local_template(&self);
    fn get_local_plugin(&self);
    fn get_local_snippet(&self);
    fn resolve(&self);
}

#[async_trait]
pub trait RemoteRegistry {
    async fn get_remote_template(&self);
    async fn get_remote_plugin(&self);
    async fn get_remote_snipet(&self);
    async fn resolve(&self);
}
