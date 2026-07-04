# DryDrop TODO

This file tracks the next concrete work items. The current priority is to make `drydrop new` generate a minimal runnable project before expanding the higher-level module system.

## P0: Make `drydrop new` Real

- [ ] Make `apps/cli/src/args/new.rs` fields accessible to `commands/new.rs`.
- [ ] Decide the MVP template engine: prefer `tera` or `minijinja` for runtime-loaded templates.
- [ ] Create a minimal `GenerationContext` from CLI args.
- [ ] Wire `apps/cli/src/commands/new.rs` into `drydrop-generator`.
- [ ] Return useful CLI output: target path, selected template, generated files.
- [ ] Add clear errors for invalid project name, missing template, and existing output directory.

## P0: Minimal Local Template

- [ ] Choose one MVP template path, likely `templates/tera/Rust/Backend/axum/`.
- [ ] Expand the template so it contains at least `Cargo.toml.tera` and `src/main.rs.tera`.
- [ ] Render `project_name` into generated files.
- [ ] Normalize generated package names so invalid Rust package names are rejected or converted safely.
- [ ] Ensure the generated Axum project can pass `cargo check`.

## P0: Filesystem Write Path

- [ ] Implement basic directory creation in `drydrop-fs`.
- [ ] Implement basic file writing in `drydrop-fs`.
- [ ] Refuse to overwrite existing files by default.
- [ ] Return a list of generated file paths from the writer.
- [ ] Add tests for writing nested directories and files.

## P1: VFS Basics

- [ ] Add constructors for `FileNode` and `DirectoryNode`.
- [ ] Add path insertion to `VfsTree`.
- [ ] Add read-only traversal for `VfsTree`.
- [ ] Add conversion from rendered template files to `VfsTree`.
- [ ] Add conversion from `VfsTree` to filesystem write operations.
- [ ] Add tests for nested file insertion, duplicate paths, and empty directories.

## P1: Generation Plan

- [ ] Define a `GenerationPlan` type.
- [ ] Represent operations as `CreateFile`, `CreateDirectory`, `Skip`, `Conflict`, and later `Patch`.
- [ ] Add a dry-run mode for `drydrop new`.
- [ ] Print a human-readable plan before writing.
- [ ] Make conflicts explicit when target files already exist.

## P1: Template Loading

- [ ] Define template metadata needed for MVP: id, name, engine, root path, variables.
- [ ] Implement a local template loader.
- [ ] Load all files recursively from a template directory.
- [ ] Render only files with the selected template extension.
- [ ] Copy non-template static files unchanged.
- [ ] Add tests for template discovery and rendering.

## P2: Manifest and Local Registry

- [ ] Design a `drydrop.toml` manifest format.
- [ ] Add manifest parsing for templates.
- [ ] Implement `LocalRegistry::resolve` for local templates.
- [ ] Remove hardcoded template lookup from the generator.
- [ ] Add registry tests with fixture templates.

## P2: Presets, Modules, and Snippets

- [ ] Define a minimal `Preset` model that references templates and default variables.
- [ ] Define a minimal `Snippet` model for additive file/content insertions.
- [ ] Extend `Module` with dependency and conflict semantics that are actually used.
- [ ] Implement dependency resolution for selected modules.
- [ ] Add conflict detection tests.

## P3: Incremental Updates

- [ ] Add `drydrop add <module>` command design.
- [ ] Implement merge strategy for `Cargo.toml`.
- [ ] Implement route/config insertion strategy for generated Rust backend projects.
- [ ] Add diff output before modifying existing projects.
- [ ] Add backup or transaction-like writes.

## P3: Hooks and Validation

- [ ] Define hook lifecycle stages.
- [ ] Add built-in post-generation formatter hook.
- [ ] Add optional generated-project smoke test hook.
- [ ] Ensure hook failures produce actionable errors.

## P4: Interfaces and Distribution

- [ ] Move high-level orchestration into `drydrop-engine`.
- [ ] Keep CLI thin and engine-driven.
- [ ] Reuse the same engine from TUI.
- [ ] Decide whether Desktop is a first-party app or later integration.
- [ ] Add local registry index commands.
- [ ] Defer remote registry until local flow is stable.

## Documentation Tasks

- [ ] Keep `README.md` aligned with actual implemented status.
- [ ] Keep `docs/zh_CN/README.md` aligned with `README.md`.
- [ ] Update `ROADMAP.md` after each milestone.
- [ ] Add an architecture note for template/render/VFS/write flow.
- [ ] Add an example generated project once MVP works.

## Suggested First PR

Goal: `cargo run -p cli -- new demo --template axum --output-dir /tmp` creates a minimal project.

Scope:

- [ ] Expose `NewArgs` fields.
- [ ] Add a simple local Tera renderer.
- [ ] Render `Cargo.toml.tera`.
- [ ] Render `src/main.rs.tera`.
- [ ] Write generated files to disk.
- [ ] Add a smoke test for generated output.

Out of scope:

- [ ] Remote registry.
- [ ] Plugin system.
- [ ] TUI/Desktop behavior.
- [ ] Incremental `drydrop add`.
- [ ] Smart merge.
