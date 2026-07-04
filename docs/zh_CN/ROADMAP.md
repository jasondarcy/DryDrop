# DryDrop 路线图

DryDrop 当前最重要的目标不是继续扩展概念，而是尽快把“选择模板 -> 渲染变量 -> 生成文件 -> 写入磁盘 -> 验证生成项目”这条最小闭环跑通。只有先有一个能工作的 `drydrop new`，后续的模块系统、VFS 合并、插件、registry、TUI/Desktop 才有可靠的落点。

## 当前判断

项目现在处于“架构骨架已搭好，生成闭环未打通”的阶段：

- `apps/cli` 已经有 `drydrop new` 的参数结构，但 `commands/new.rs` 仍是空实现。
- `drydrop-core` 已有 `Project`、`Module`、`Variables` 等领域模型雏形。
- `drydrop-generator` 已有 `GenerationContext`、`GenerationOutput` 和 `Generator` 外壳，但还没有真正加载模板、渲染和写文件。
- `drydrop-template` 可以表达模板和变量，但还没有模板加载、渲染接口。
- `drydrop-vfs` 有 `Node`、`DirectoryNode`、`FileNode`、`VfsTree`，但还缺少路径插入、遍历和落盘能力。
- `drydrop-fs` 目前只有 reader/writer 模块占位。
- `templates/` 下有 Axum 的 Askama/Tera 实验模板，适合作为第一个 MVP 目标。

因此路线应当从“做出一个可运行生成器”开始，而不是先做完整插件系统或远程 registry。

## Phase 0: 基础整理

目标：让当前骨架更容易继续开发。

- 明确 CLI 参数、生成上下文和输出目录语义。
- 让 `NewArgs` 字段对命令实现可读。
- 统一命名：`project_name`、`template`、`output_dir`。
- 确认 MVP 使用一个模板引擎。建议先选 `tera` 或 `minijinja`，因为它们适合运行时读取模板文件。
- 暂时保留 Askama 作为实验，不把它作为第一条 MVP 主线。

完成标准：

- `cargo check --workspace` 通过。
- `cargo run -p cli -- new demo --template axum` 能进入真实命令逻辑。
- 错误信息能说明模板不存在、输出目录不可写等基本问题。

## Phase 1: MVP 生成闭环

目标：实现最小可用的 `drydrop new`。

范围只包含一个本地模板：`templates/tera/Rust/Backend/axum/`。

生成流程：

1. CLI 接收 `name`、`template`、`output_dir`。
2. 构造 `GenerationContext`。
3. 从本地模板目录读取文件。
4. 渲染模板变量，例如 `project_name`。
5. 写入 `output_dir/name`。
6. 打印生成结果 summary。
7. 对生成项目运行 smoke test，例如 `cargo check`。

完成标准：

- `cargo run -p cli -- new my-api --template axum --output-dir /tmp` 能生成项目。
- 生成目录至少包含 `Cargo.toml` 和 `src/main.rs`。
- 生成项目可执行 `cargo check`。
- 重复生成到已有目录时默认失败，而不是覆盖。

## Phase 2: VFS 与生成计划

目标：把“直接写文件”升级为“先计划，后写入”。

需要补齐：

- `VfsTree` 支持插入文件和目录。
- `Node` 支持遍历。
- 模板渲染结果先进入 VFS。
- VFS 可以转成写入计划。
- 写入计划可以展示为 `create`、`overwrite`、`skip`、`conflict`。
- CLI 增加 dry-run 或 plan 输出。

完成标准：

- `drydrop new` 默认仍能写入。
- 内部生成流程经过 VFS。
- 用户可以看到将创建哪些文件。
- 已存在文件不会被静默覆盖。

## Phase 3: 模板 Manifest 与 Registry

目标：让模板不再依赖硬编码路径。

需要设计：

- `drydrop.toml` 或类似 manifest。
- Template metadata：id、name、language、category、engine、entry path、variables。
- LocalRegistry：扫描本地模板目录，按 id 查找模板。
- RemoteRegistry 继续保持 trait，占位即可。

完成标准：

- CLI 不再硬编码 Axum 路径。
- `--template axum` 可通过 LocalRegistry 解析。
- 模板缺少变量、模板路径无效、渲染失败时有清晰错误。

## Phase 4: 模块、Preset 与 Snippet

目标：从“生成一个模板”演进到“组合多个能力”。

建议顺序：

1. 先做 preset：把一组模板/变量组合成命名方案。
2. 再做 snippet：向已有 VFS 中插入小片段。
3. 最后做 module：加入依赖、冲突、分类、版本、合并策略。

示例：

- `preset rust/axum/api`
- `snippet axum/route`
- `module db/postgres`
- `module auth/session`

完成标准：

- `drydrop new my-api --preset rust/axum/api` 能工作。
- `drydrop add route users` 可以在 VFS 层表达计划。
- 模块依赖冲突能被检测出来。

## Phase 5: 增量更新与安全合并

目标：支持已有项目继续添加能力。

关键问题：

- `Cargo.toml` 不能简单覆盖，需要结构化合并。
- 路由注册、配置文件、环境变量示例需要内容级插入。
- 用户手写代码必须被保护。
- 所有写入前都应能查看 diff。

完成标准：

- `drydrop add db/postgres` 不覆盖已有项目。
- 对冲突文件生成 reviewable diff。
- 写入前有备份或事务机制。

## Phase 6: Hook、Plugin 与多端界面

目标：把 DryDrop 从工具变成平台。

内容：

- Hook 生命周期：`pre_resolve`、`post_resolve`、`pre_render`、`post_render`、`pre_write`、`post_write`。
- 内置 Hook：格式化、运行测试、检查生成项目。
- Plugin API：先用 Rust trait，本地插件优先，远程插件后置。
- TUI/Desktop 复用 `drydrop-engine`，不要复制 CLI 逻辑。

完成标准：

- CLI 和 TUI 可以调用同一套 engine。
- 生成后可自动运行 formatter 或 smoke test。
- 插件失败有明确错误上下文。

## 推荐近期路线

接下来 3 个最值得做的任务：

1. **先让 `drydrop new` 真正写出最小 Axum 项目。**
2. **再把写入流程改成 VFS + plan。**
3. **最后设计 manifest 和 LocalRegistry，解除模板路径硬编码。**

不要太早做远程 registry、复杂插件系统或 Desktop UI。它们依赖稳定的生成核心，过早实现会让抽象变重。
