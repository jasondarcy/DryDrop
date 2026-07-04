# DryDrop

[English](README.md) | [Chinese](docs/zh_CN/README.md)

> Composable full-stack project generator, written in Rust.

DryDrop is an early-stage project generator for assembling maintainable full-stack application skeletons from reusable building blocks. The long-term goal is to make project creation and project evolution feel like selecting capabilities, not copying boilerplate: choose a backend framework, persistence layer, auth strategy, UI stack, deployment target, hooks, and snippets, then let DryDrop resolve the plan and write the project safely.

DryDrop is currently a foundation and planning-stage implementation. The workspace already contains the application entry points, core domain crates, registry/generator abstractions, VFS pieces, filesystem helpers, and initial Axum template experiments. The real generation pipeline is still being built.

## Project Goals

- **Composable generation**: Build projects from presets, templates, snippets, hooks, and modules instead of one large template.
- **Safe incremental changes**: Support both `new` project generation and future `add`/`upgrade` operations on existing projects.
- **Clear module contracts**: Track module metadata, variables, dependencies, conflicts, and generated files explicitly.
- **In-memory planning first**: Resolve templates into a virtual file tree before touching disk.
- **Template engine flexibility**: Keep template rendering pluggable; the workspace currently experiments with `askama`, `tera`, and `minijinja`.
- **Extensible automation**: Allow hooks and plugins to participate in lifecycle stages such as resolving, rendering, writing, and post-generation checks.
- **Multiple interfaces**: Start with the CLI, then expose the same engine to a TUI and desktop app.

## Non-Goals

- DryDrop is not intended to be a framework or runtime.
- DryDrop should not hide the generated code behind a proprietary abstraction.
- DryDrop should not require a remote registry for local templates and presets to work.
- DryDrop should not overwrite user projects blindly; generation must become inspectable and reversible.

## Target User Experience

The intended flow is:

```bash
drydrop new my-api --preset rust/axum
drydrop add database/postgres
drydrop add auth/session
drydrop plan
drydrop apply
```

The first working milestone is smaller:

```bash
cargo run -p cli -- new my-api --template axum
```

That command should create a minimal, runnable Rust backend project from a local template.

## Architecture

DryDrop is organized as a Cargo workspace. User-facing applications live in `apps/`; reusable project-generation logic lives in `crates/`.

```text
.
├── apps/
│   ├── cli/                 # CLI entry point
│   ├── tui/                 # Terminal UI placeholder
│   └── desktop/             # Desktop app placeholder
│
├── crates/
│   ├── drydrop-core/        # Project, module, variable, and error models
│   ├── drydrop-generator/   # Generation context, planning, resolving, pipeline, output
│   ├── drydrop-template/    # Template model and template-owned VFS tree
│   ├── drydrop-registry/    # Local/remote registry traits
│   ├── drydrop-vfs/         # Virtual file tree, nodes, directories, files, merge traits
│   ├── drydrop-fs/          # Filesystem reader/writer helpers
│   ├── drydrop-diff/        # Future diff/patch support
│   ├── drydrop-plugin/      # Future plugin extension points
│   ├── drydrop-hook/        # Hook abstractions
│   ├── drydrop-preset/      # Preset abstractions
│   ├── drydrop-snippet/     # Snippet abstractions
│   └── drydrop-engine/      # Future high-level orchestration facade
│
├── templates/
│   ├── askama/Rust/Backend/axum/
│   └── tera/Rust/Backend/axum/
└── docs/
    └── zh_CN/
        ├── README.md
        ├── ROADMAP.md
        └── TODO.md
```

## Concept Model

DryDrop is centered around a few composable concepts:

- **Project**: The target application being generated, including name, output directory, and selected modules.
- **Template**: A source file tree with variables and metadata that can render into a generated project.
- **Preset**: A curated selection of templates/modules, such as `rust/axum/api`.
- **Module**: A capability that can be added to a project, such as `web/axum`, `db/postgres`, or `auth/session`.
- **Snippet**: A smaller reusable insertion, such as a route, middleware, config section, or CI job.
- **Hook**: Lifecycle logic that can validate, transform, format, or run checks around generation.
- **Registry**: A local or remote catalog for discovering templates, presets, snippets, and plugins.
- **VFS**: The in-memory file tree used to plan and merge output before writing to disk.

## Planned Data Flow

```text
CLI / TUI / Desktop
        │
        ▼
GenerationContext
        │
        ▼
Registry resolution
        │
        ▼
Dependency and conflict resolver
        │
        ▼
Template rendering + snippet expansion
        │
        ▼
VFS assembly and merge plan
        │
        ▼
Diff / review / confirmation
        │
        ▼
Safe filesystem write
        │
        ▼
Hooks: format, test, post-generate checks
```

## Current Status

| Area | Status | Notes |
| --- | --- | --- |
| CLI | Skeleton | `drydrop new` is parsed but command logic is still stubbed. |
| Core models | Early | `Project`, `Module`, `Variables`, and domain wrappers exist. |
| Generator | Early | `GenerationContext`, `GenerationOutput`, and `Generator` shell exist. |
| Registry | Interface | Local/remote registry traits exist; implementations are pending. |
| Template | Early model | Template owns a module, VFS root, and variables. |
| VFS | Early model | Tree/node modules exist; merge behavior needs implementation. |
| FS helpers | Partial | Reader/writer modules exist. Backup/transaction semantics are pending. |
| Templates | Experiments | Axum templates exist under Askama and Tera paths. |
| TUI/Desktop | Placeholder | Application crates exist but no product flow yet. |

## Roadmap

### Milestone 1: Runnable Local Template

- Implement `drydrop new <name> --template axum`.
- Load a local Axum template from `templates/`.
- Render project variables such as `project_name`.
- Write files to the selected output directory.
- Add a smoke test that generated projects can run `cargo check`.

### Milestone 2: Generation Plan and VFS

- Generate into `drydrop-vfs` first.
- Convert VFS output into filesystem operations.
- Add overwrite policy: fail, overwrite, skip, or prompt.
- Introduce a human-readable `drydrop plan` output.

### Milestone 3: Presets, Modules, and Snippets

- Define manifest formats for templates, presets, modules, and snippets.
- Resolve module dependencies and conflicts.
- Support small additive snippets such as routes, config files, and CI workflows.
- Separate one-shot project templates from incremental project modifications.

### Milestone 4: Safe Incremental Updates

- Add `drydrop add <module>`.
- Implement merge strategies for common files such as `Cargo.toml`, routes, config, and environment examples.
- Add diff/patch review before writing.
- Add backups or transactional writes.

### Milestone 5: Hooks, Registry, and Interfaces

- Implement hook lifecycle stages.
- Add a local registry index.
- Add remote registry support later.
- Reuse the same engine from CLI, TUI, and desktop app.

## Development

### Requirements

- Rust toolchain, preferably managed with [mise](https://mise.jdx.dev/).
- This workspace uses Rust edition 2024.

### Common Commands

```bash
cargo check --workspace
cargo test --workspace
cargo build --workspace
cargo run -p cli -- --help
cargo run -p cli -- new my-app --template axum
```

### First Implementation Target

The most useful next change is to wire `apps/cli/src/commands/new.rs` into `drydrop-generator`:

1. Parse `NewArgs` into a `GenerationContext`.
2. Resolve the requested local template.
3. Render template variables.
4. Build a VFS tree.
5. Write the tree to `output_dir/project_name`.
6. Return a clear summary of generated files.

## Documentation

Chinese documentation: [`docs/zh_CN/README.md`](docs/zh_CN/README.md)

Planning documents:

- [`ROADMAP.md`](ROADMAP.md) describes the phased product and engineering route.
- [`TODO.md`](TODO.md) tracks the next concrete implementation tasks.

## License

[MIT](LICENSE) © 2026 NotNOne
