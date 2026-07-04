# DryDrop

[English](../../README.md) | [简体中文](README.md)

> 用 Rust 编写的可组合全栈项目生成器。

DryDrop 的目标是做一个“可以持续演进项目”的生成器，而不仅是一次性复制模板的脚手架。理想状态下，用户可以选择预设、模板、模块、代码片段和 Hook，例如后端框架、数据库、认证方式、UI 栈、部署目标和 CI 配置，DryDrop 负责解析依赖、规划文件树、生成代码、展示差异，并安全写入项目目录。

当前项目仍处于早期设计和骨架阶段。仓库已经具备 Cargo workspace、CLI/TUI/Desktop 应用入口、核心领域模型、生成器抽象、注册表 trait、VFS 雏形、文件系统辅助模块，以及 Axum 模板实验；真正可用的生成流水线还在建设中。

## 项目愿景

DryDrop 想解决的是全栈项目初始化和长期维护中的重复劳动：

- 新项目经常需要重复搭建目录结构、依赖、配置、路由、数据库连接、认证、日志、测试和 CI。
- 传统脚手架通常只负责“第一次生成”，后续添加模块仍要人工改多个文件。
- 大模板容易变得臃肿，不同技术栈组合之间难以复用。
- 生成工具如果直接写磁盘，容易覆盖用户代码，缺少计划、差异和回滚能力。

因此 DryDrop 的核心方向是：先把用户选择转成一份可解释的生成计划，再在内存 VFS 中合成目标文件树，最后通过安全写入、差异审查和 Hook 完成落盘。

## 产品目标

- **项目生成**：从本地或远程模板生成一个可运行的项目。
- **模块组合**：通过模块声明组合 Web、DB、Auth、UI、Deploy、CI 等能力。
- **预设方案**：用 preset 表达常见技术栈，例如 `rust/axum/api`。
- **增量添加**：未来支持在已有项目中执行 `drydrop add database/postgres`。
- **安全写入**：写入前先生成计划和 VFS，后续支持 diff、备份和事务式写入。
- **模板引擎可插拔**：当前 workspace 依赖中包含 `tera`、`minijinja`，模板目录也保留了 `askama` 实验。
- **Hook 与插件**：允许在解析、生成、写入、格式化、测试等阶段扩展行为。
- **多端入口**：CLI 先行，后续 TUI 和 Desktop 复用同一个核心引擎。

## 非目标

- DryDrop 不是 Web 框架，不接管生成项目的运行时。
- DryDrop 不应该把生成代码藏在黑盒里；生成结果应当清晰、可读、可维护。
- DryDrop 不应该强依赖远程服务；本地模板、预设和模块必须能独立工作。
- DryDrop 不应该默认覆盖用户已有代码；所有破坏性操作都应当可审查。

## 目标使用体验

长期希望支持类似流程：

```bash
drydrop new my-api --preset rust/axum
drydrop add database/postgres
drydrop add auth/session
drydrop plan
drydrop apply
```

近期第一个可交付目标更小：

```bash
cargo run -p cli -- new my-api --template axum
```

这个命令应当从本地模板生成一个最小可运行的 Rust Axum 项目，并能在生成目录中通过 `cargo check`。

## 核心概念

- **Project**：目标项目，包含项目名、输出目录、已选择模块等信息。
- **Template**：模板文件树，包含变量和元数据，渲染后形成项目文件。
- **Preset**：预设组合，用来表达常见项目方案，例如 Rust Axum API 服务。
- **Module**：可组合能力，例如 `web/axum`、`db/postgres`、`auth/session`。
- **Snippet**：比模块更小的可复用插入，例如路由、配置段、CI job、middleware。
- **Hook**：生成生命周期中的扩展点，可用于校验、格式化、测试或后处理。
- **Registry**：模板、预设、模块、插件的索引来源，可以是本地或远程。
- **VFS**：虚拟文件系统，在真正写磁盘前承载生成结果和合并计划。

## 仓库结构

```text
.
├── apps/
│   ├── cli/                 # CLI 入口，当前已有 drydrop new 参数解析
│   ├── tui/                 # TUI 占位应用
│   └── desktop/             # Desktop 占位应用
│
├── crates/
│   ├── drydrop-core/        # Project、Module、Variables、Error 等领域模型
│   ├── drydrop-generator/   # GenerationContext、Output、Resolver、Planner、Pipeline
│   ├── drydrop-template/    # Template 模型，连接 Module、Variables 和 VFS
│   ├── drydrop-registry/    # LocalRegistry / RemoteRegistry trait
│   ├── drydrop-vfs/         # 虚拟文件树、节点、目录、文件和合并 trait
│   ├── drydrop-fs/          # 文件系统 reader/writer 辅助模块
│   ├── drydrop-diff/        # 后续 diff/patch 能力
│   ├── drydrop-plugin/      # 后续插件系统
│   ├── drydrop-hook/        # Hook 抽象
│   ├── drydrop-preset/      # Preset 抽象
│   ├── drydrop-snippet/     # Snippet 抽象
│   └── drydrop-engine/      # 后续高层编排 facade
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

## 计划中的生成流程

```text
CLI / TUI / Desktop 接收用户选择
        │
        ▼
构建 GenerationContext
        │
        ▼
Registry 解析模板、预设、模块、片段
        │
        ▼
依赖解析与冲突检查
        │
        ▼
模板渲染与 Snippet 展开
        │
        ▼
组装 VFS 并生成写入计划
        │
        ▼
展示 diff / plan / 用户确认
        │
        ▼
安全写入文件系统
        │
        ▼
执行 Hook：格式化、测试、后处理
```

## 当前状态

| 区域 | 状态 | 说明 |
| --- | --- | --- |
| CLI | 骨架 | `drydrop new` 参数解析存在，命令实现仍为空。 |
| Core | 早期模型 | 已有 `Project`、`Module`、`Variables` 等基础类型。 |
| Generator | 早期模型 | 已有 `GenerationContext`、`GenerationOutput`、`Generator` 外壳。 |
| Registry | trait | 已定义本地/远程 registry trait，尚无实际实现。 |
| Template | 早期模型 | Template 关联 Module、VFS root 和变量列表。 |
| VFS | 早期模型 | Tree/Node/File/Directory 模块存在，合并逻辑待完善。 |
| FS | 部分模块 | reader/writer 模块存在，备份和事务语义待补。 |
| Templates | 实验 | 当前有 Axum 的 Askama/Tera 模板实验。 |
| TUI/Desktop | 占位 | crate 存在，产品流程未实现。 |

## 里程碑规划

### 1. 最小可用生成器

目标：让 `drydrop new my-api --template axum` 真正生成一个可运行项目。

- 将 `apps/cli/src/commands/new.rs` 接入 `drydrop-generator`。
- 把 CLI 参数转换为 `GenerationContext`。
- 实现本地模板加载。
- 渲染 `project_name` 等基础变量。
- 写入 `output_dir/project_name`。
- 增加生成结果 summary。
- 增加 smoke test，确认生成项目可 `cargo check`。

### 2. VFS 与计划输出

目标：先计划，后写入。

- 生成结果先落到 `drydrop-vfs`。
- VFS 转换成文件系统操作计划。
- 支持写入策略：失败、覆盖、跳过、询问。
- 增加 `drydrop plan`，展示将创建、修改、跳过的文件。
- 为后续 diff/patch 打基础。

### 3. 模板、预设、模块、片段

目标：从“一个模板”演进到“可组合生成”。

- 设计 manifest 格式，描述 template/preset/module/snippet。
- 定义模块依赖和冲突规则。
- 区分大模板和小片段：模板负责项目骨架，snippet 负责增量插入。
- 先支持 Rust Backend Axum，再扩展 Actix、前端、数据库、部署等能力。

### 4. 增量更新

目标：支持已有项目继续添加能力。

- 实现 `drydrop add <module>`。
- 为常见文件提供合并策略，例如 `Cargo.toml`、路由文件、配置文件、`.env.example`。
- 引入 diff 审查。
- 写入前备份，失败时可回滚。

### 5. Hook、Registry 与多端入口

目标：把生成器做成可扩展平台。

- 定义 Hook 生命周期：解析前后、生成前后、写入前后、完成后检查。
- 实现本地 registry index。
- 后续加入远程 registry。
- TUI/Desktop 复用同一个 `drydrop-engine`。

## 开发命令

```bash
cargo check --workspace
cargo test --workspace
cargo build --workspace
cargo run -p cli -- --help
cargo run -p cli -- new my-app --template axum
```

## 最近最值得做的实现

当前最关键的下一步是把 CLI 和生成器串起来：

1. 让 `NewArgs` 的字段对命令实现可见。
2. 在 `commands/new.rs` 中创建 `GenerationContext`。
3. 实现一个最小本地模板加载器。
4. 选择一个模板引擎作为 MVP，建议先用 `tera` 或 `minijinja`，因为它们适合运行时加载模板文件。
5. 渲染 `templates/tera/Rust/Backend/axum/` 下的最小模板。
6. 使用 `drydrop-fs` 写入文件。
7. 为 `drydrop new` 添加端到端测试。

## 设计原则

- 生成器核心应尽量与 CLI 解耦，方便 TUI/Desktop 复用。
- 模板文件结构应尽量接近最终项目结构，降低维护成本。
- 领域模型先保持简单，不要过早引入复杂插件 ABI。
- 所有会修改磁盘的能力都应先能 dry-run。
- 默认行为应保守，宁可失败也不要静默覆盖用户代码。

## 相关资源

- [Clap](https://github.com/clap-rs/clap) - CLI 参数解析
- [Snafu](https://github.com/shepmaster/snafu) - 错误处理
- [Tera](https://github.com/Keats/tera) - 运行时模板引擎
- [MiniJinja](https://github.com/mitsuhiko/minijinja) - 轻量模板引擎
- [Askama](https://github.com/djc/askama) - 编译期模板引擎

## 英文文档

请查看 [`../../README.md`](../../README.md)。

## 规划文档

- [`../../ROADMAP.md`](../../ROADMAP.md)：英文阶段性产品与工程路线。
- [`../../TODO.md`](../../TODO.md)：英文下一步可执行任务清单。
- [`ROADMAP.md`](ROADMAP.md)：中文阶段性产品与工程路线。
- [`TODO.md`](TODO.md)：中文下一步可执行任务清单。
