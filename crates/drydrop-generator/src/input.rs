use drydrop_error::{DryDropError, DryDropResult};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GenerationInput {
    pub name: String,
    pub template: String,
    pub destination: PathBuf,
    pub templates_root: PathBuf,
}

impl GenerationInput {
    pub fn project_dir(&self) -> PathBuf {
        self.destination.join(&self.name)
    }

    pub fn validate(&self) -> DryDropResult<()> {
        validate_project_name(&self.name)?;

        if !self.templates_root.is_dir() {
            return Err(DryDropError::UnsupportedTemplate(format!(
                "templates root not found: {}",
                self.templates_root.display()
            )));
        }

        Ok(())
    }
}

pub fn validate_project_name(name: &str) -> DryDropResult<()> {
    if name.is_empty() {
        return Err(DryDropError::InvalidProjectName(
            "project name cannot be empty".into(),
        ));
    }

    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Err(DryDropError::InvalidProjectName(
            "project name cannot be empty".into(),
        ));
    };

    if !first.is_ascii_alphanumeric() {
        return Err(DryDropError::InvalidProjectName(format!(
            "'{name}' must start with an ASCII letter or digit"
        )));
    }

    if !name
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Err(DryDropError::InvalidProjectName(format!(
            "'{name}' may only contain ASCII letters, digits, hyphens, and underscores"
        )));
    }

    Ok(())
}

pub fn find_templates_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;

    loop {
        let candidate = dir.join("templates/tera/Rust");
        if candidate.is_dir() {
            return Some(candidate);
        }

        if !dir.pop() {
            break;
        }
    }

    None
}

pub fn resolve_template_dir(templates_root: &Path, template: &str) -> DryDropResult<PathBuf> {
    let candidates = [
        templates_root.join("backend").join(template),
        templates_root.join(template),
    ];

    for candidate in candidates {
        if candidate.is_dir() {
            return Ok(candidate);
        }
    }

    Err(DryDropError::UnsupportedTemplate(format!(
        "template '{template}' not found under {}",
        templates_root.display()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_project_names() {
        assert!(validate_project_name("my-api").is_ok());
        assert!(validate_project_name("my_api").is_ok());
        assert!(validate_project_name("api2").is_ok());
    }

    #[test]
    fn rejects_invalid_project_names() {
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("-bad").is_err());
        assert!(validate_project_name("bad name").is_err());
        assert!(validate_project_name("bad.name").is_err());
    }

    #[test]
    fn resolves_axum_and_barebone_templates() {
        let root = find_templates_root().expect("templates should exist in repo");

        assert!(resolve_template_dir(&root, "axum").is_ok());
        assert!(resolve_template_dir(&root, "barebone").is_ok());
        assert!(resolve_template_dir(&root, "missing").is_err());
    }
}
