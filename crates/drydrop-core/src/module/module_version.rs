pub struct ModuleVersion(String);

impl ModuleVersion {
    pub fn new() -> Self {
        Self(String::new())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
