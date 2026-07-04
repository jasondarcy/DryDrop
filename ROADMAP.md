# DryDrop Roadmap

DryDrop's most important near-term goal is not to expand the concept surface, but to complete the smallest useful generation loop: select a template, render variables, produce files, write them safely, and verify the generated project. Once `drydrop new` works end to end, modules, VFS merging, plugins, registries, TUI, and desktop interfaces will have a concrete foundation.

## Current Assessment

The project is currently in the "architecture scaffold exists, generation loop is not wired yet" stage:

- `apps/cli` already defines the `drydrop new` argument shape, but `commands/new.rs` is still a stub.
- `drydrop-core` has early domain models such as `Project`, `Module`, and `Variables`.
- `drydrop-generator` has `GenerationContext`, `GenerationOutput`, and a `Generator` shell, but it does not yet load templates, render files, or write output.
- `drydrop-template` can represent a template and variables, but template loading and rendering are not implemented.
- `drydrop-vfs` has `Node`, `DirectoryNode`, `FileNode`, and `VfsTree`, but still needs path insertion, traversal, and write-plan conversion.
- `drydrop-fs` currently only has reader/writer placeholders.
- `templates/` contains Axum experiments for Askama and Tera, which are good candidates for the first MVP.

The route should start by making a working generator, not by implementing a complete plugin system or remote registry.

## Phase 0: Foundation Cleanup

Goal: make the current scaffold easier to implement against.

- Clarify CLI argument, generation context, and output directory semantics.
- Make `NewArgs` fields readable from the command implementation.
- Standardize naming around `project_name`, `template`, and `output_dir`.
- Pick one MVP template engine. Prefer `tera` or `minijinja` first because they support runtime template loading well.
- Keep Askama as an experiment for now, not the first MVP path.

Done when:

- `cargo check --workspace` passes.
- `cargo run -p cli -- new demo --template axum` enters real command logic.
- Basic errors explain missing templates, invalid names, and unwritable output directories.

## Phase 1: MVP Generation Loop

Goal: implement the smallest useful `drydrop new`.

Scope: one local template, likely `templates/tera/Rust/Backend/axum/`.

Flow:

1. CLI receives `name`, `template`, and `output_dir`.
2. Build a `GenerationContext`.
3. Read files from the local template directory.
4. Render variables such as `project_name`.
5. Write output to `output_dir/name`.
6. Print a generation summary.
7. Run a smoke test such as `cargo check` on the generated project.

Done when:

- `cargo run -p cli -- new my-api --template axum --output-dir /tmp` creates a project.
- The generated project contains at least `Cargo.toml` and `src/main.rs`.
- The generated project can run `cargo check`.
- Re-generating into an existing directory fails by default instead of overwriting.

## Phase 2: VFS and Generation Plan

Goal: move from direct file writes to "plan first, write second".

Required work:

- Add file and directory insertion to `VfsTree`.
- Add traversal to `Node` / `VfsTree`.
- Render templates into VFS before touching disk.
- Convert VFS output into a write plan.
- Represent operations such as `create`, `overwrite`, `skip`, and `conflict`.
- Add dry-run or plan output to the CLI.

Done when:

- `drydrop new` still writes output.
- The internal flow goes through VFS.
- Users can see which files will be created.
- Existing files are not silently overwritten.

## Phase 3: Template Manifest and Registry

Goal: stop depending on hardcoded template paths.

Design:

- A `drydrop.toml` or similar manifest format.
- Template metadata: id, name, language, category, engine, entry path, and variables.
- LocalRegistry scans local template directories and resolves templates by id.
- RemoteRegistry remains a trait until local flow is stable.

Done when:

- The CLI no longer hardcodes the Axum template path.
- `--template axum` resolves through LocalRegistry.
- Missing variables, invalid template paths, and render failures produce clear errors.

## Phase 4: Modules, Presets, and Snippets

Goal: evolve from "generate one template" into "compose capabilities".

Recommended order:

1. Implement presets: named combinations of templates and default variables.
2. Implement snippets: small additive insertions into a VFS.
3. Implement modules: dependencies, conflicts, categories, versions, and merge strategies.

Examples:

- `preset rust/axum/api`
- `snippet axum/route`
- `module db/postgres`
- `module auth/session`

Done when:

- `drydrop new my-api --preset rust/axum/api` works.
- `drydrop add route users` can be expressed as a VFS-level plan.
- Module dependency conflicts can be detected.

## Phase 5: Incremental Updates and Safe Merging

Goal: support adding capabilities to existing projects.

Key problems:

- `Cargo.toml` cannot be overwritten; it needs structured merging.
- Routes, config files, and environment examples need content-level insertion.
- User-written code must be protected.
- Every write should be reviewable through a diff.

Done when:

- `drydrop add db/postgres` does not overwrite an existing project.
- Conflicting files produce reviewable diffs.
- Writes have backup or transaction-like safety.

## Phase 6: Hooks, Plugins, and Interfaces

Goal: turn DryDrop from a tool into an extensible platform.

Work:

- Define hook stages: `pre_resolve`, `post_resolve`, `pre_render`, `post_render`, `pre_write`, `post_write`.
- Add built-in hooks for formatting, smoke tests, and generated-project checks.
- Start with Rust trait-based local plugins before remote plugins.
- Make TUI/Desktop call `drydrop-engine` instead of duplicating CLI logic.

Done when:

- CLI and TUI can call the same engine.
- Generated projects can be formatted or checked automatically.
- Hook failures include clear context.

## Recommended Near-Term Route

The next three highest-value tasks are:

1. **Make `drydrop new` generate a minimal Axum project.**
2. **Move the write path behind VFS and a generation plan.**
3. **Design manifest and LocalRegistry support to remove hardcoded template paths.**

Avoid building remote registry, complex plugins, or Desktop UI too early. Those depend on a stable generation core and will make the architecture heavy if implemented before the MVP loop exists.
