use indexmap::IndexMap;
use drydrop_snippet::Snippet;

pub struct SnippetRepository {
    pub snippets: IndexMap<String, Snippet>,
}

impl SnippetRepository {
    pub fn new() -> Self {
        Self {
            snippets: IndexMap::new(),
        }
    }
}

impl Default for SnippetRepository {
    fn default() -> Self {
        Self {
            snippets: IndexMap::new(),
        }
    }
}
