# DryDrop TODO

这个文件记录中文任务清单。当前优先级是先让 `drydrop new` 能生成一个最小可运行项目，再扩展更高层的模块系统。

## P0: 让 `drydrop new` 真正工作

- [ ] 让 `apps/cli/src/args/new.rs` 中的字段能被 `commands/new.rs` 读取。
- [ ] 确定 MVP 模板引擎：优先考虑 `tera` 或 `minijinja`，因为它们适合运行时加载模板。
- [ ] 根据 CLI 参数创建最小 `GenerationContext`。
- [ ] 将 `apps/cli/src/commands/new.rs` 接入 `drydrop-generator`。
- [ ] 返回有用的 CLI 输出：目标路径、选中模板、生成文件列表。
- [ ] 为非法项目名、模板不存在、输出目录已存在等情况提供清晰错误。

## P0: 最小本地模板

- [ ] 选择一个 MVP 模板路径，建议使用 `templates/tera/Rust/Backend/axum/`。
- [ ] 扩展模板，至少包含 `Cargo.toml.tera` 和 `src/main.rs.tera`。
- [ ] 渲染 `project_name` 到生成文件中。
- [ ] 规范化生成的 package name，非法 Rust 包名应拒绝或安全转换。
- [ ] 确保生成的 Axum 项目可以通过 `cargo check`。

## P0: 文件系统写入路径

- [ ] 在 `drydrop-fs` 中实现基础目录创建。
- [ ] 在 `drydrop-fs` 中实现基础文件写入。
- [ ] 默认拒绝覆盖已有文件。
- [ ] 从 writer 返回生成文件路径列表。
- [ ] 为嵌套目录和文件写入添加测试。

## P1: VFS 基础

- [ ] 为 `FileNode` 和 `DirectoryNode` 添加构造函数。
- [ ] 为 `VfsTree` 添加按路径插入文件/目录的能力。
- [ ] 为 `VfsTree` 添加只读遍历能力。
- [ ] 支持把渲染后的模板文件转换为 `VfsTree`。
- [ ] 支持把 `VfsTree` 转换为文件系统写入操作。
- [ ] 为嵌套文件插入、重复路径、空目录添加测试。

## P1: 生成计划

- [ ] 定义 `GenerationPlan` 类型。
- [ ] 用 `CreateFile`、`CreateDirectory`、`Skip`、`Conflict`、后续 `Patch` 表达操作。
- [ ] 为 `drydrop new` 增加 dry-run 模式。
- [ ] 写入前打印可读的生成计划。
- [ ] 目标文件已存在时显式报告冲突。

## P1: 模板加载

- [ ] 定义 MVP 所需模板元数据：id、name、engine、root path、variables。
- [ ] 实现本地模板加载器。
- [ ] 递归加载模板目录下所有文件。
- [ ] 只渲染选定模板扩展名的文件。
- [ ] 非模板静态文件原样复制。
- [ ] 为模板发现和渲染添加测试。

## P2: Manifest 与本地 Registry

- [ ] 设计 `drydrop.toml` manifest 格式。
- [ ] 增加模板 manifest 解析。
- [ ] 实现本地模板的 `LocalRegistry::resolve`。
- [ ] 从 generator 中移除硬编码模板查找。
- [ ] 使用 fixture 模板添加 registry 测试。

## P2: Preset、Module 与 Snippet

- [ ] 定义最小 `Preset` 模型，引用模板和默认变量。
- [ ] 定义最小 `Snippet` 模型，用于增量文件/内容插入。
- [ ] 扩展 `Module`，让依赖和冲突语义真正被使用。
- [ ] 实现选中模块的依赖解析。
- [ ] 添加冲突检测测试。

## P3: 增量更新

- [ ] 设计 `drydrop add <module>` 命令。
- [ ] 实现 `Cargo.toml` 合并策略。
- [ ] 实现 Rust 后端项目的路由/配置插入策略。
- [ ] 修改已有项目之前输出 diff。
- [ ] 增加备份或事务式写入。

## P3: Hook 与校验

- [ ] 定义 Hook 生命周期阶段。
- [ ] 增加内置生成后格式化 Hook。
- [ ] 增加可选的生成项目 smoke test Hook。
- [ ] 确保 Hook 失败能产生可操作错误。

## P4: 多端入口与分发

- [ ] 将高层编排移动到 `drydrop-engine`。
- [ ] 保持 CLI 轻量，由 engine 驱动。
- [ ] TUI 复用同一套 engine。
- [ ] 决定 Desktop 是一等应用还是后续集成。
- [ ] 增加本地 registry index 命令。
- [ ] 在本地流程稳定前暂缓远程 registry。

## 文档任务

- [ ] 保持根目录 `README.md` 与实际状态一致。
- [ ] 保持 `docs/zh_CN/README.md` 与英文 README 对齐。
- [ ] 每完成一个里程碑后更新 `ROADMAP.md` 和 `docs/zh_CN/ROADMAP.md`。
- [ ] 增加 template/render/VFS/write flow 的架构说明。
- [ ] MVP 完成后添加一个生成项目示例。

## 建议第一个 PR

目标：让 `cargo run -p cli -- new demo --template axum --output-dir /tmp` 能生成最小项目。

范围：

- [ ] 暴露 `NewArgs` 字段。
- [ ] 添加一个简单的本地 Tera renderer。
- [ ] 渲染 `Cargo.toml.tera`。
- [ ] 渲染 `src/main.rs.tera`。
- [ ] 将生成文件写入磁盘。
- [ ] 为生成结果添加 smoke test。

不包含：

- [ ] 远程 registry。
- [ ] 插件系统。
- [ ] TUI/Desktop 行为。
- [ ] 增量 `drydrop add`。
- [ ] 智能合并。
