pub struct FileName(String);


impl FileName {
    pub fn value(&self) -> &str {
        &self.0
    }
}
