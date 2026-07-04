use crate::error::{DryDropResult};
use crate::error::DryDropError::InvalidProjectName;

pub struct ProjectName(String);

impl ProjectName {
    pub fn new(project_name: impl Into<String>) -> DryDropResult<Self> {
        let project_name = project_name.into();
        if project_name.is_empty() {
            return Err(InvalidProjectName(project_name))
        }
        Ok(Self(project_name))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
