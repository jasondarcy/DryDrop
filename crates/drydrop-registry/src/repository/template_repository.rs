use indexmap::IndexMap;
use drydrop_template::Template;

pub struct TemplateRepository {
    pub templates: IndexMap<String, Template>,
}

impl TemplateRepository {
    pub fn new() -> Self {
        Self {
            templates: IndexMap::new(),
        }
    }
}

impl Default for TemplateRepository {
    fn default() -> Self {
        Self {
            templates: IndexMap::new(),
        }
    }
}
