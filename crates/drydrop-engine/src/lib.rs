pub enum Engine {
    Tera(tera::Context),
}

impl Engine {
    pub fn new() -> Self {
        Self::Tera(tera::Context::new())
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
