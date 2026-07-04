pub struct ModuleName(String);

impl ModuleName {
    pub fn new() -> Self {
        Self(String::new())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
