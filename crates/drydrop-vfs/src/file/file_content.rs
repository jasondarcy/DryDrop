pub struct FileContent(String);

impl FileContent {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
