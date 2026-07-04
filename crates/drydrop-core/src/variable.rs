use std::collections::HashMap;

pub struct Variables(HashMap<String, String>);

impl Variables {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn value(&self) -> &HashMap<String, String> {
        &self.0
    }
}
