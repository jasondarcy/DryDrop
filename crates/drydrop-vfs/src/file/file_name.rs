pub struct FileName(String);

impl FileName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
