# MIT 课程大纲

> 用 Rust 实现迷你 Git — 10 节课学习计划

## 课程信息

- **总课时**: 10 节课
- **项目地址**: https://github.com/MrBeanCpp/mit
- **前置知识**: 基本的 Rust 语法

---

## 课程目标

通过实现一个迷你 Git，理解版本控制系统的核心原理：
- 对象存储模型
- 暂存区机制
- 分支管理
- 变更追踪

---

## 课程大纲

### 第 1 课：Git 原理与项目概述

**内容**:
- Git 的核心设计思想
- Git 内部数据结构（对象、引用、暂存区）
- MIT 项目结构介绍
- 开发环境搭建

**源码阅读**:
- `Cargo.toml` — 依赖项
- `src/main.rs` — 程序入口

**动手实验**:
- 克隆项目并运行 `cargo run -- --help`

---

### 第 2 课：仓库初始化与存储结构

**内容**:
- `.mit/` 目录结构
- 对象存储（`.mit/objects/`）
- 引用存储（`.mit/refs/heads/`）
- HEAD 指针（`.mit/HEAD`）

**源码阅读**:
- `src/commands/init.rs` — 初始化仓库
- `src/utils/store.rs` — 对象存储管理

**重点概念**:
```
ROOT_DIR = ".mit"
├── objects/    # 对象存储（blob/tree/commit）
├── refs/heads/ # 分支引用
└── HEAD        # 当前指针
```

**动手实验**:
- 运行 `cargo run -- init`
- 查看 `.mit/` 目录结构

---

### 第 3 课：Hash 对象与 SHA-1 存储

**内容**:
- SHA-1 哈希算法
- Hash 类型定义
- 对象的唯一标识

**源码阅读**:
- `src/models/object.rs` — Hash 类型定义
- `src/utils/store.rs` — calc_hash() 方法

**重点代码**:
```rust
pub type Hash = String;  // SHA-1 哈希值

fn calc_hash(data: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
```

---

### 第 4 课：Blob 对象 — 文件内容存储

**内容**:
- Blob 是 Git 最基本的对象
- 文件内容压缩存储
- Base64 编码
- Blob 的创建与读取

**源码阅读**:
- `src/models/blob.rs`

**重点概念**:
```
Blob = 文件内容 + SHA-1 Hash
```

**动手实验**:
- 创建文件并添加到仓库
- 查看 `.mit/objects/` 中的存储

---

### 第 5 课：Tree 对象 — 目录结构

**内容**:
- Tree 包含多个 TreeEntry
- TreeEntry = 文件名 + Hash + 权限模式
- 递归构建目录树
- Blob 与 Tree 的关系

**源码阅读**:
- `src/models/tree.rs`

**重点概念**:
```
Tree
├── TreeEntry (blob) → file1.txt
├── TreeEntry (blob) → file2.txt
└── TreeEntry (tree) → subdir/
    ├── TreeEntry (blob) → file3.txt
    └── ...
```

---

### 第 6 课：Commit 对象 — 提交记录

**内容**:
- Commit 的组成（tree、parent、message、author）
- 初始提交与后续提交
- Commit 的创建与保存
- 提交历史与 parent 链

**源码阅读**:
- `src/models/commit.rs`

**重点概念**:
```rust
struct Commit {
    tree: String,           // Tree Hash
    parent: Vec<Hash>,       // 父提交
    message: String,
    author: String,
    date: SystemTime,
}
```

---

### 第 7 课：Index — 暂存区机制

**内容**:
- Index 的作用（.mit/index）
- 暂存区 vs 工作区 vs HEAD
- 文件元数据（FileMetaData）
- Index 的加载与保存

**源码阅读**:
- `src/models/index.rs`

**重点概念**:
```
工作区 (Working Directory)
    ↓ add
暂存区 (Index/Staging Area)
    ↓ commit
对象库 (Objects)
```

---

### 第 8 课：add 与 commit 命令

**内容**:
- `mit add` 实现流程
  - 计算文件 Hash
  - 创建 Blob
  - 更新 Index
- `mit commit` 实现流程
  - 创建 Tree
  - 创建 Commit
  - 更新 HEAD

**源码阅读**:
- `src/commands/add.rs` — add 命令
- `src/commands/commit.rs` — commit 命令

**动手实验**:
- 创建文件 → add → commit
- 查看每个步骤中 `.mit/` 的变化

---

### 第 9 课：status 与变更追踪

**内容**:
- 三种变更状态：new / modified / deleted
- `changes_to_be_staged()` — 工作区 vs 暂存区
- `changes_to_be_committed()` — 暂存区 vs HEAD
- Untracked 文件

**源码阅读**:
- `src/commands/status.rs`

**重点概念**:
```
Changes to be committed:  暂存区 vs HEAD
Changes not staged:       工作区 vs 暂存区
Untracked files:         工作区中从未跟踪的文件
```

---

### 第 10 课：分支与合并

**内容**:
- 分支的本质（refs/heads/branch_name）
- HEAD 的两种模式（Branch / Detached）
- `mit branch` — 创建、删除、列出分支
- `mit switch` — 切换分支
- `mit merge` — Fast-Forward 合并
- 合并冲突检测

**源码阅读**:
- `src/models/head.rs` — HEAD 管理
- `src/commands/branch.rs` — 分支命令
- `src/commands/switch.rs` — 切换命令
- `src/commands/merge.rs` — 合并命令
- `src/commands/restore.rs` — 恢复命令

**重点概念**:
```
Branch = refs/heads/<name> 指向某个 Commit
HEAD (Branch) = ref: refs/heads/master
HEAD (Detached) = <commit_hash>
```

**动手实验**:
- 创建分支 → 切换 → 提交 → 合并
- 观察 `.mit/` 目录的变化

---

## 每课对应的源码文件

| 课程 | 核心源码 |
|------|----------|
| 第 1 课 | `src/main.rs` |
| 第 2 课 | `src/commands/init.rs`, `src/utils/store.rs` |
| 第 3 课 | `src/models/object.rs`, `src/utils/store.rs` |
| 第 4 课 | `src/models/blob.rs` |
| 第 5 课 | `src/models/tree.rs` |
| 第 6 课 | `src/models/commit.rs` |
| 第 7 课 | `src/models/index.rs` |
| 第 8 课 | `src/commands/add.rs`, `src/commands/commit.rs` |
| 第 9 课 | `src/commands/status.rs` |
| 第 10 课 | `src/models/head.rs`, `src/commands/branch.rs`, `src/commands/switch.rs`, `src/commands/merge.rs`, `src/commands/restore.rs` |

---

## 推荐学习顺序

```
第1课 → 第2课 → 第3课 → 第4课 → 第5课 → 第6课 → 第7课 → 第8课 → 第9课 → 第10课
```

**为什么这样安排**:
- 第 1-2 课建立整体概念
- 第 3-6 课学习核心数据模型（由底向上）
- 第 7 课理解暂存区
- 第 8-9 课学习命令实现
- 第 10 课综合运用

---

## 扩展学习

完成本课程后，可以进一步学习：
- [Mega-Libra](https://github.com/web3infra-foundation/mega/tree/main/libra) — 更完善的 Git 实现
- [Pro Git Book](https://git-scm.com/book/zh/v2) — 官方 Git 文档

---

## 运行测试

```bash
# 单线程运行所有测试
cargo test -- --test-threads=1
```
