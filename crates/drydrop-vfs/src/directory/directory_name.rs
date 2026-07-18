pub struct DirectoryName(String);

impl DirectoryName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn root() -> Self {
        Self(String::new())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
