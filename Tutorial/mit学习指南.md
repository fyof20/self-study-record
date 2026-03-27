# MIT 学习指南

> Mini-Git in Rust 学习笔记

## 项目简介

MIT 是一个用 Rust 实现的迷你 Git，旨在简洁易读、高效且安全。
适合作为学习 Git 内部原理的入门项目。

**仓库地址**: https://github.com/MrBeanCpp/mit

## 学习路径

### 第一阶段：先了解 Git 核心概念

按以下顺序学习核心数据结构：

| 概念 | 对应文件 | 说明 |
|------|----------|------|
| Hash | `src/models/object.rs` | 对象的 SHA-1 标识 |
| Blob | `src/models/blob.rs` | 文件内容存储（对应 Git blob object） |
| Tree | `src/models/tree.rs` | 目录结构（对应 Git tree object） |
| Commit | `src/models/commit.rs` | 提交记录（包含 tree hash、parent、message） |
| Index | `src/models/index.rs` | 暂存区（.mit/index） |
| Head | `src/models/head.rs` | 当前分支指针 |

### 第二阶段：学习命令实现

按从简单到复杂的顺序学习命令：

```
init → add → status → commit → log → rm → restore → branch → switch → merge
```

对应文件：
- `src/commands/init.rs`
- `src/commands/add.rs`
- `src/commands/status.rs`
- `src/commands/commit.rs`
- `src/commands/log.rs`
- `src/commands/remove.rs` (rm)
- `src/commands/restore.rs`
- `src/commands/branch.rs`
- `src/commands/switch.rs`
- `src/commands/merge.rs`

### 第三阶段：理解存储机制

- `src/utils/store.rs` - 对象存储（.mit/objects）
- `src/utils/path_ext.rs` - 路径处理
- `src/utils/util.rs` - 通用工具函数

## 核心文件索引

| 模块 | 路径 | 作用 |
|------|------|------|
| 入口 | `src/main.rs` | CLI 入口 |
| 命令解析 | `src/cli.rs` | 命令行参数处理 |
| 命令实现 | `src/commands/*.rs` | 各个命令的具体实现 |
| 核心模型 | `src/models/` | Blob/Tree/Commit 等数据结构 |
| 工具函数 | `src/utils/` | 存储、路径等工具 |

## 学习方法建议

### 1. 先跑通项目
```bash
cargo run -- --help
cargo run -- init
cargo run -- status
```

### 2. 边学边调试
- 用 `cargo run -- init` 创建仓库
- 用 `cargo run -- status` 查看状态
- 用 `cargo run -- log` 查看提交历史
- 用 `cargo run -- add <file>` 添加文件

### 3. 对照官方文档
建议阅读：
- [Git 内部原理](https://git-scm.com/book/zh/v2/Git-内部原理)
- [Pro Git Book](https://git-scm.com/book/zh/v2)

### 4. 理解存储结构
重点观察 `.mit/` 目录下发生了什么：
- `.mit/objects/` - 对象存储
- `.mit/refs/` - 分支引用
- `.mit/HEAD` - 当前指针
- `.mit/index` - 暂存区

## 这个项目覆盖的 Git 核心概念

- **对象模型**: blob / tree / commit 三种对象
- **暂存区机制**: index 文件的作用
- **分支指针**: HEAD 如何指向当前分支
- **快速前进合并**: fast-forward merge 的实现
- **文件变更跟踪**: 新建、修改、删除的处理

## 运行测试

> 注意：测试需要单线程运行，避免冲突。

```bash
cargo test -- --test-threads=1
```

## 后续学习

更完善的 Git 实现可参考：
- [Mega-Libra](https://github.com/web3infra-foundation/mega/tree/main/libra)

## 推荐的学习顺序

1. 运行 `cargo run -- --help` 了解支持的命令
2. 阅读 README 了解项目结构
3. 学习 `src/models/` 中的核心数据结构
4. 学习 `src/commands/init.rs` 了解仓库初始化
5. 学习 `src/commands/add.rs` 和 `src/commands/commit.rs` 理解暂存和提交
6. 学习 `src/commands/status.rs` 理解工作区、暂存区、HEAD 的比较
7. 学习 `src/commands/branch.rs` 和 `src/commands/switch.rs` 理解分支
8. 学习 `src/commands/merge.rs` 理解合并

祝学习愉快！
