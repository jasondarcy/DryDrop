use crate::args::new::NewArgs;
use crate::models::vcs::Vcs;
use drydrop_generator::{find_templates_root, generate_project, GenerationInput};
use eyre::{bail, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run(args: NewArgs) -> Result<()> {
    let templates_root = find_templates_root()
        .ok_or_else(|| eyre::eyre!("could not find templates directory (templates/tera/Rust)"))?;

    let input = GenerationInput {
        name: args.name().to_string(),
        template: args
            .template()
            .unwrap_or("barebone")
            .to_string(),
        destination: args
            .destination()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".")),
        templates_root,
    };

    let output = generate_project(&input).map_err(|err| eyre::eyre!("{err}"))?;

    if matches!(args.vcs(), Vcs::Git) {
        init_git(&output.project_dir)?;
    }

    println!("Generated project: {}", output.project_dir.display());
    println!("Template: {}", output.template);
    println!("Files:");

    for file in &output.files {
        let relative = file
            .strip_prefix(&output.project_dir)
            .unwrap_or(file);
        println!("  {}", relative.display());
    }

    Ok(())
}

fn init_git(project_dir: &std::path::Path) -> Result<()> {
    let status = Command::new("git")
        .args(["init"])
        .current_dir(project_dir)
        .status()?;

    if !status.success() {
        bail!("failed to initialize git repository in {}", project_dir.display());
    }

    Ok(())
}
