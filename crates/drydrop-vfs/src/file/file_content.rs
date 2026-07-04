pub struct FileContent(String);

impl FileContent {
    pub fn value(&self) -> &str {
        &self.0
    }
}
