use crate::args::new::NewArgs;
use drydrop_fs::writer::FsWriter;
use eyre::{Context, Result, bail};
use std::fs;
use tera::Tera;

pub fn run(args: NewArgs) -> Result<()> {
    if args.template() != "axum" {
        bail!(
            "unsupported template '{}'; only 'axum' is supported for now",
            args.template()
        );
    }

    let output_base = args.output_dir().unwrap_or_else(|| ".".as_ref());
    let project_dir = output_base.join(args.name());

    if project_dir.exists() {
        bail!("Output directory already exists: {}", project_dir.display());
    }

    let template_root = "templates/tera/Rust/Backend/axum";
    let cargo_template = format!("{template_root}/Cargo.toml.tera");
    let main_template = format!("{template_root}/src/main.rs.tera");

    let mut context = tera::Context::new();
    context.insert("project_name", args.name());

    let cargo_toml = Tera::one_off(
        &fs::read_to_string(&cargo_template)
            .with_context(|| format!("failed to read template: {cargo_template}"))?,
        &context,
        false,
    )?;

    let main_rs = Tera::one_off(
        &fs::read_to_string(&main_template)
            .with_context(|| format!("failed to read template: {main_template}"))?,
        &context,
        false,
    )?;

    let generated_files = vec![
        FsWriter::write_file(project_dir.join("Cargo.toml"), cargo_toml)?,
        FsWriter::write_file(project_dir.join("src/main.rs"), main_rs)?,
    ];

    println!("Generated project: {}", project_dir.display());
    println!("Template: {}", args.template());
    println!("Files:");

    for file in generated_files {
        let relative_path = file.strip_prefix(&project_dir).unwrap_or(&file);
        println!("  {}", relative_path.display());
    }

    Ok(())
}
