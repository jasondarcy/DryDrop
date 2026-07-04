pub struct ModuleId(String);

impl ModuleId {
    pub fn new() -> Self {
        Self(String::new())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
