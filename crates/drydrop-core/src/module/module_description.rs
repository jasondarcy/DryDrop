pub struct ModuleDescription(String);

impl ModuleDescription {
    pub fn new() -> Self {
        Self(String::new())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
