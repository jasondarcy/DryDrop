use crate::input::{resolve_template_dir, GenerationInput};
use crate::output::GenerationOutput;
use drydrop_error::{DryDropError, DryDropResult};
use drydrop_fs::writer::FsWriter;
use std::fs;
use std::path::{Path, PathBuf};
use tera::Tera;

pub fn generate_project(input: &GenerationInput) -> DryDropResult<GenerationOutput> {
    input.validate()?;

    let template_dir = resolve_template_dir(&input.templates_root, &input.template)?;
    let project_dir = input.project_dir();

    if project_dir.exists() {
        return Err(DryDropError::InvalidProjectOutputDir(
            project_dir.display().to_string(),
        ));
    }

    let mut context = tera::Context::new();
    context.insert("project_name", &input.name);

    let files = render_template_dir(&template_dir, &project_dir, &context)?;

    if !project_dir.exists() {
        fs::create_dir_all(&project_dir)?;
    }

    Ok(GenerationOutput::new(
        project_dir,
        input.template.clone(),
        files,
    ))
}

fn render_template_dir(
    template_dir: &Path,
    project_dir: &Path,
    context: &tera::Context,
) -> DryDropResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_and_render(template_dir, template_dir, project_dir, context, &mut files)?;
    Ok(files)
}

fn collect_and_render(
    root: &Path,
    current: &Path,
    project_dir: &Path,
    context: &tera::Context,
    files: &mut Vec<PathBuf>,
) -> DryDropResult<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_and_render(root, &path, project_dir, context, files)?;
            continue;
        }

        let relative = path
            .strip_prefix(root)
            .map_err(|_| DryDropError::Io(std::io::Error::other("invalid template path")))?;

        let output_relative = strip_tera_extension(relative);
        let output_path = project_dir.join(output_relative);
        let template_content = fs::read_to_string(&path)?;

        let rendered = if path.extension().is_some_and(|ext| ext == "tera") {
            Tera::one_off(&template_content, context, false)
                .map_err(|err| DryDropError::TemplateRender(err.to_string()))?
        } else {
            template_content
        };

        let written = FsWriter::write_file(output_path, rendered)?;
        files.push(written);
    }

    Ok(())
}

fn strip_tera_extension(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if let Some(trimmed) = file_name.strip_suffix(".tera") {
        path.with_file_name(trimmed)
    } else {
        path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::find_templates_root;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_name(prefix: &str) -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        format!("{prefix}-{nanos}")
    }

    #[test]
    fn generates_axum_project_from_local_templates() {
        let templates_root = find_templates_root().expect("templates should exist in repo");
        let destination = std::env::temp_dir().join("drydrop-test");
        fs::create_dir_all(&destination).expect("temp destination should be creatable");

        let name = unique_name("drydrop-axum");
        let input = GenerationInput {
            name: name.clone(),
            template: "axum".into(),
            destination: destination.clone(),
            templates_root,
        };

        let output = generate_project(&input).expect("axum project should generate");
        assert!(output.project_dir.join("Cargo.toml").is_file());
        assert!(output.project_dir.join("src/main.rs").is_file());
        assert_eq!(output.files.len(), 2);

        let cargo_toml = fs::read_to_string(output.project_dir.join("Cargo.toml"))
            .expect("Cargo.toml should be readable");
        assert!(cargo_toml.contains(&format!("name = \"{name}\"")));

        let _ = fs::remove_dir_all(output.project_dir);
    }

    #[test]
    fn generates_empty_project_from_empty_template() {
        let templates_root = find_templates_root().expect("templates should exist in repo");
        let destination = std::env::temp_dir().join("drydrop-test");
        fs::create_dir_all(&destination).expect("temp destination should be creatable");

        let name = unique_name("drydrop-actix");
        let input = GenerationInput {
            name: name.clone(),
            template: "actix".into(),
            destination: destination.clone(),
            templates_root,
        };

        let output = generate_project(&input).expect("actix project should generate");
        assert!(output.project_dir.is_dir());
        assert_eq!(output.files.len(), 0);

        let _ = fs::remove_dir_all(output.project_dir);
    }
}
