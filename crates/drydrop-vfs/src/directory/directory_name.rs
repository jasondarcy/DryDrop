pub struct DirectoryName(String);

impl DirectoryName {
    pub fn new() -> Self {
        Self(String::new())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
