# crab-shell — 软件工程文档

> **crab-shell**：Rust 编写的数据库 Schema 文档交付工具。蟹壳是螃蟹的外层描述，正如文档是数据库的外层描述。shell 既是蟹壳，也是命令行。

**文档版本**：v3.1
**创建日期**：2026-05-13
**文档类型**：软件工程设计（个人开发项目）
**基于**：可行性分析报告（2026-05-13）

---

## 文档修订历史

| 版本 | 日期 | 修改内容 | 作者 |
|------|------|----------|------|
| v1.0 | 2026-05-13 | 初始版本（Java + 团队开发） | - |
| v2.0 | 2026-05-13 | 全面重构：Rust 替代 Java、个人开发项目、WSL Ubuntu 开发环境 | 叶彬弘 |
| v3.0 | 2026-05-13 | 引入 DDD 模块化开发 + TDD 测试驱动 + 全栈容器化（Docker Compose） | 叶彬弘 |
| v3.1 | 2026-05-13 | 项目命名为 crab-shell，全局替换项目名和 CLI 命令 | 叶彬弘 |

---

## 目录

1. [项目概述](#一项目概述)
2. [开发流程计划](#二开发流程计划)
3. [技术选型](#三技术选型)
4. [开发环境搭建](#四开发环境搭建)
5. [系统架构设计](#五系统架构设计)
6. [功能设计](#六功能设计)
7. [非功能性设计](#七非功能性设计)
8. [CLI/TUI 交互设计](#八clitui-交互设计)
9. [测试策略](#九测试策略)
10. [部署方案](#十部署方案)
11. [项目里程碑](#十一项目里程碑)
12. [风险评估与应对](#十二风险评估与应对)
13. [附录](#十三附录)

---

## 一、项目概述

### 1.1 项目背景

在外包项目交付过程中，数据库文档的整理是一项耗时且容易出错的工作。根据可行性调研，外包团队每次交付文档需要耗时 **4-12 小时**，且存在文档与实际 Schema 不同步的问题。

现有工具（如 pgAdmin、Navicat、Screw 等）主要关注数据库管理或开发辅助，**没有任何工具专注于"交付包打包"场景**——即一键生成包含 ER 图、数据字典、变更记录的可分享文档包。

### 1.2 产品定位

**一句话定位**：PostgreSQL Schema 快照文档生成器——一键生成 ER 图 + 数据字典 + 变更记录，打包成可分享的交付文档包。

**核心差异化**：
- 唯一能一键打包 ER 图 + 数据字典 + 变更记录为可分享文档的产品
- 专注"交付场景"，而非"开发场景"
- 与 Shield/pgAdmin 不冲突（它们做"改"，本工具做"读 + 打包"）

### 1.3 目标用户

| 用户群 | 场景 | 核心痛点 | 付费意愿 |
|--------|------|----------|----------|
| **外包/交付团队（最优先）** | 项目交付时需要完整数据库文档 | 手工整理耗时 4-12 小时/次 | 高 |
| **中小团队后端开发者** | 新人入职了解数据库，前后端联调 | 口头交接/截图，信息不统一 | 中 |

### 1.4 项目目标

**MVP 目标（5-8 天，个人开发）**：
- ✅ 支持 PostgreSQL 数据库连接（只读）
- ✅ 生成 Markdown 格式数据字典
- ✅ 生成 Mermaid ER 图（按 schema 分组）
- ✅ 生成当前 Schema 的 JSON 快照
- ✅ 提供 CLI 入口（`crab-shell generate`）

**长期目标**：
- 支持 HTML/PDF 输出
- 支持 Schema Diff / 变更记录
- TUI 交互界面（Rust 独特优势）
- 支持多数据库（MySQL、SQL Server 等）

### 1.5 成功指标

| 指标 | MVP 目标 | v1.0 目标 |
|------|-----------|-----------|
| **功能完整性** | 核心功能可用 | 包含所有计划功能 |
| **文档生成时间** | < 30 秒（< 50 表） | < 10 秒 |
| **二进制体积** | < 10 MB | < 15 MB |
| **冷启动时间** | < 100ms | < 100ms |
| **内存占用** | < 50 MB | < 50 MB |

### 1.6 个人开发项目说明

本项目为 **个人独立开发项目**，开发者具备以下背景：

- 编程语言：Python、Rust（学习中）
- 开发环境：WSL Ubuntu（VSCode / CodeBuddy 远程连接）
- 版本控制：Git + GitHub
- 部署环境：Linux 服务器 / Docker

**个人开发的优势**：
- 无沟通成本，技术决策即时执行
- 代码风格统一，架构一致性高
- 灵活调整优先级，快速迭代

**个人开发的挑战**：
- 需要自行把控所有技术决策
- 时间有限，需合理规划优先级
- 需要 CI/CD 自动化弥补缺少 QA 的问题

---

## 二、开发流程计划

### 2.1 开发方法论

个人项目采用 **轻量级敏捷（Lean Agile）** + **DDD（领域驱动设计）** + **TDD（测试驱动开发）** 方法，核心原则：

- **短迭代**：以 2-3 天为一个开发周期，每次完成一个可运行的功能增量
- **主线开发**：在 `main` 分支上直接开发（个人项目无需复杂分支策略），里程碑时打 tag
- **DDD 模块化思维**：按领域（Domain）划分模块，而非按技术层划分；每个领域内聚自己的数据模型、业务逻辑和接口
- **TDD 黑箱思维**：先写测试定义行为契约（"它应该做什么"），再写实现代码（"怎么做到"）；测试驱动设计，而非验证实现
- **容器化先行**：所有依赖服务（PostgreSQL、MySQL、MongoDB）通过 Docker Compose 管理，开发环境与 CI/CD 保持一致
- **文档随代码同步**：每个迭代结束时更新 README 和变更日志

#### 2.1.1 DDD 模块化设计原则

本项目虽然体量不大，但采用 DDD 思想组织代码有以下好处：
- **概念清晰**：`schema` 领域、`generator` 领域、`snapshot` 领域各自独立，新增数据库类型只需扩展 `schema` 领域
- **天然可扩展**：未来支持 MySQL/MongoDB 时，只需新增对应适配器（Adapter），不侵入已有代码
- **测试友好**：每个领域可以独立测试，mock 最小化

**领域划分**：

```
┌─────────────────────────────────────────────────────────┐
│                      应用层 (Application)                 │
│              CLI 入口、配置解析、任务编排                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌───────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ Schema 领域    │  │ Generator 领域│  │ Snapshot 领域│ │
│  │ (领域模型)     │  │ (文档生成)    │  │ (快照管理)   │ │
│  │               │  │              │  │              │ │
│  │ - Table       │  │ - Markdown   │  │ - Save/Load  │ │
│  │ - Column      │  │ - Mermaid    │  │ - Diff       │ │
│  │ - Constraint  │  │ - HTML       │  │ - Compare    │ │
│  │ - Index       │  │              │  │              │ │
│  └───────┬───────┘  └──────────────┘  └──────────────┘ │
│          │                                               │
├──────────┼───────────────────────────────────────────────┤
│          ▼              基础设施层 (Infrastructure)        │
│  ┌───────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ PostgreSQL    │  │ MySQL (未来)  │  │ MongoDB(未来)│ │
│  │ Adapter       │  │ Adapter      │  │ Adapter      │ │
│  └───────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────┘
```

#### 2.1.2 TDD 工作循环

每个功能模块严格遵循 **Red → Green → Refactor** 循环：

```
  1. Red（红）
     ┌──────────────────────────────────────┐
     │  编写测试，定义期望行为（黑箱思维）      │
     │  - 先确定"输入什么，期望输出什么"       │
     │  - 不考虑实现细节                      │
     │  → 运行测试，确认失败（红灯）          │
     └──────────────────┬───────────────────┘
                      ▼
  2. Green（绿）
     ┌──────────────────────────────────────┐
     │  编写最小实现代码，使测试通过           │
     │  - 不追求完美，只追求通过              │
     │  → 运行测试，确认通过（绿灯）          │
     └──────────────────┬───────────────────┘
                      ▼
  3. Refactor（重构）
     ┌──────────────────────────────────────┐
     │  在测试保护下重构代码                  │
     │  - 消除重复、优化命名、改善结构         │
     │  → 运行测试，确认仍然通过              │
     └──────────────────────────────────────┘
```

**TDD 实践要点**：
- ✅ 核心逻辑（Schema 读取、Diff 引擎、文档生成）**必须先写测试**
- ✅ 测试命名描述行为：`test_should_return_empty_tables_for_empty_schema`
- ✅ 测试即文档：读测试就能理解模块的契约
- ✅ 容器化测试：所有依赖 PostgreSQL/MySQL/MongoDB 的测试使用 testcontainers 自动拉起容器
- ❌ 不写无意义的覆盖率代码（getter/setter 不需要测试）

### 2.2 开发阶段划分

#### 阶段一：MVP（5-8 天）

**目标**：验证核心假设，提供可用的最小功能集。

| 天数 | 核心任务 | TDD/容器化要求 | 交付物 | 验收标准 |
|------|----------|---------------|--------|----------|
| **Day 1** | Docker Compose 搭建 + Cargo 初始化 + clap CLI 骨架 | 先写 CLI 参数解析测试 | 可运行的空壳 CLI + 容器化环境 | `docker compose up` 正常；`crab-shell --help` 正常输出 |
| **Day 2** | PostgreSQL 适配器 + 读取表/列 | 先写容器化集成测试（testcontainers） | 数据库连接和基础读取 | 能连接容器中的测试数据库并打印表清单 |
| **Day 3** | 完善 Schema 读取（主键、外键、索引、注释） | 先写各类型读取测试 | 完整的 Schema 读取模块 | 读取结果与 pgAdmin 一致 |
| **Day 4** | Markdown 数据字典生成 | 先写 Markdown 输出快照测试 | Markdown 文档输出 | 生成的 .md 文件在 GitHub 正确渲染 |
| **Day 5** | Mermaid ER 图生成 | 先写 ER 语法正确性测试 | ER 图输出 | 生成的 Mermaid 语法正确 |
| **Day 6** | JSON 快照生成/加载 + CLI 参数完善 | 先写序列化往返测试 | JSON 快照功能 | 快照可保存和重新加载 |
| **Day 7** | 错误处理优化 + 帮助文档 + README | 补充错误场景测试 | 完整 MVP | 错误信息友好，README 可用 |
| **Day 8** | 全量测试通过 + 容器化 CI 验证 + Bug 修复 | CI 中使用 service container | 稳定 MVP | `cargo test` 全部通过（含容器化集成测试） |

**关键里程碑**：
- [ ] Day 1：`docker compose up -d` 启动 PostgreSQL 容器；`crab-shell --help` 输出帮助信息
- [ ] Day 2：testcontainers 自动拉起 PostgreSQL 容器，成功读取表清单
- [ ] Day 3：完整 Schema 信息读取（含注释、约束、索引），容器化测试通过
- [ ] Day 4：Markdown 输出快照测试通过
- [ ] Day 5：Mermaid 语法正确性测试通过
- [ ] Day 6：JSON 序列化往返测试通过
- [ ] Day 7：CLI 体验完善（进度条、彩色输出、友好错误）
- [ ] Day 8：全量测试通过 + README 完成

#### 阶段二：v1.0 — 可交互文档（3-5 天）

**目标**：支持 HTML 输出，提供可交互的文档体验。

| 天数 | 核心任务 | 交付物 |
|------|----------|--------|
| **Day 1-2** | 集成 Tera 模板引擎 + 设计默认 HTML 模板 | HTML 文档输出 |
| **Day 3** | 嵌入 Mermaid.js 渲染（可交互 ER 图） | 可缩放/搜索的 ER 图 |
| **Day 4** | 自定义模板支持 + 搜索功能 | 模板系统 |
| **Day 5** | 测试 + README 更新 + Release 构建 | v1.0 发布 |

#### 阶段三：v1.1 — 变更追踪（3-5 天）

**目标**：支持 Schema Diff，自动生成变更记录。

| 天数 | 核心任务 | TDD 要求 | 交付物 |
|------|----------|----------|--------|
| **Day 1-2** | JSON Snapshot 对比引擎 | 先写各种 diff 场景测试（增表、删列、改类型等） | SchemaDiff 模块 |
| **Day 3** | Markdown 格式变更记录生成 | 先写变更记录格式测试 | 变更报告 |
| **Day 4** | GitHub Actions CI 集成（容器化服务） | CI 使用 service container 运行 PostgreSQL | CI/CD Pipeline |
| **Day 5** | 测试 + README 更新 | 容器化集成测试全部通过 | v1.1 发布 |

#### 阶段四：v1.2 — 正式交付（2-3 天）

**目标**：支持 PDF 输出，完善交付流程。

| 天数 | 核心任务 | 交付物 |
|------|----------|--------|
| **Day 1** | HTML → PDF 转换（调用 wkhtmltopdf 或内置 headless Chrome） | PDF 输出 |
| **Day 2** | 品牌定制（Logo、配色） + 批量生成 | 定制功能 |
| **Day 3** | 测试 + 文档 + Release | v1.2 发布 |

### 2.3 任务分解（WBS）

```
1. 项目准备（Day 1，0.5 天）
   1.1 cargo init + DDD 目录结构设计（按领域划分）
   1.2 依赖配置（Cargo.toml）
   1.3 Docker Compose 编排（PostgreSQL/MySQL/MongoDB 容器）
   1.4 Git 仓库初始化 + .gitignore
   1.5 CI/CD 基础配置（GitHub Actions + service containers）
   1.6 clap CLI 骨架搭建

2. 核心功能开发（Day 2-6，4 天）—— 按 DDD 领域组织
   2.1 Schema 领域 — 数据模型层（Day 2-3，1.5 天）
       2.1.1 领域模型定义（DatabaseSchema/Table/Column struct + serde）
       2.1.2 PostgreSQL 适配器（tokio-postgres 连接 + information_schema 查询）
       2.1.3 pg_catalog 扩展查询（注释、枚举）
       2.1.4 约束读取（主键、外键、唯一、检查）
       2.1.5 索引读取
       2.2 Generator 领域 — 文档生成层（Day 4-5，1.5 天）
       2.2.1 DocumentGenerator trait（领域接口）
       2.2.2 Markdown 生成器
       2.2.3 Mermaid ER 图生成器
       2.2.4 HTML 生成器（v1.0）
       2.3 Snapshot 领域 — 快照管理层（Day 6，0.5 天）
       2.3.1 快照序列化/反序列化
       2.3.2 快照文件管理
       2.3.3 Schema Diff 引擎（v1.1）

3. 基础设施层（贯穿开发周期）
   3.1 连接管理（异步、重试、超时）
   3.2 容器化测试基础设施（testcontainers 封装）
   3.3 错误处理（thiserror + miette + anyhow）

4. CLI 体验完善（Day 6-7，0.5 天）
   4.1 彩色输出（indicatif + console）
   4.2 进度条（大量表时）
   4.3 友好错误信息（miette）
   4.4 帮助文档 + 示例

5. 测试与发布（Day 8，1 天）—— TDD 贯穿
   5.1 单元测试（核心领域模型）
   5.2 容器化集成测试（PostgreSQL + testcontainers）
   5.3 快照测试（确定性输入/输出对比）
   5.4 交叉编译（x86_64 + aarch64）
   5.5 README 编写
   5.6 GitHub Release 构建
```

### 2.4 交付物清单

| 阶段 | 交付物 | 格式 |
|------|--------|------|
| **MVP** | 源代码 | GitHub 仓库 |
| **MVP** | 静态链接二进制 | `crab-shell`（Linux x86_64 / macOS / Windows） |
| **MVP** | README + 使用文档 | Markdown |
| **v1.0** | HTML 模板系统 | Tera 模板 |
| **v1.0** | 可交互文档示例 | HTML |
| **v1.1** | Schema Diff 引擎 | Rust 模块 |
| **v1.2** | PDF 输出模块 | Rust 模块 |

---

## 三、技术选型

### 3.1 技术选型原则

1. **Rust 优先**：充分利用 Rust 的性能、安全和单二进制优势
2. **异步友好**：数据库 I/O 使用 async/await，保持响应速度
3. **零运行时依赖**：编译为静态二进制，用户下载即用
4. **CLI 原生**：选择专门为 CLI 优化的 crate
5. **DDD 模块化**：按领域（Domain）组织 crate 模块，高内聚低耦合
6. **容器化一致**：开发、测试、CI/CD 环境全部基于 Docker Compose，消除环境差异

### 3.2 核心技术栈

#### 3.2.1 语言与运行时

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **编程语言** | Rust | 1.77+ (edition 2021) | 单二进制分发、零运行时依赖、内存安全、CLI 工具生态成熟 |
| **异步运行时** | Tokio | 1.x | Rust 异步生态事实标准，tokio-postgres 依赖 |
| **Rust 版本管理** | rustup | latest | 管理 Rust 工具链 |

**为什么选 Rust 而非 Java**：

| 维度 | Java 17 | Rust |
|------|---------|------|
| 启动时间 | ~1-2s（JVM 预热） | **< 50ms** |
| 内存占用 | **100-300 MB**（JVM） | 2-10 MB |
| 二进制体积 | ~50 MB（fat JAR） | **3-8 MB**（静态链接） |
| 分发体验 | 需要用户装 JRE | **单文件零依赖** |
| Docker 镜像 | ~250 MB | **~20 MB** |
| CLI 体验 | 启动慢，体感重 | **瞬间启动，轻量** |
| TUI 生态 | 几乎没有 | **ratatui（活跃、社区大）** |
| 开发速度（本项目估算） | 3-5 天 | **5-8 天** |

**结论**：Rust 在开发速度上多投入 2-3 天，但换来的是更好的用户体验和更清晰的产品演进路径（CLI → TUI）。作为个人项目，长期维护效率和产品口碑比短期开发速度更重要。

#### 3.2.2 CLI 框架

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **CLI 参数解析** | clap | 4.x | derive 宏驱动，自动生成 `--help`，Rust CLI 生态标准 |
| **进度条** | indicatif | 0.17+ | 美观的进度条和 spinner |
| **彩色输出** | console / termcolor | 0.15+ | 跨平台彩色终端输出 |
| **错误报告** | miette | 5.x | 漂亮的诊断错误报告，类似 Rust 编译器的错误提示 |
| **交互式提示** | dialoguer / inquire | - | 未来 TUI / 交互式模式使用 |

#### 3.2.3 数据库访问

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **异步 PostgreSQL 驱动** | tokio-postgres | 0.7+ | 原生 async，性能好，无需 FFI |
| **连接字符串解析** | postgres-derive | - | 简化连接配置 |

**核心查询**（与 Java 版相同，使用标准 SQL）：

```sql
-- 读取表清单
SELECT table_name, table_type, table_schema
FROM information_schema.tables
WHERE table_schema = $1
ORDER BY table_name;

-- 读取列详情
SELECT column_name, data_type, is_nullable, column_default,
       character_maximum_length, ordinal_position
FROM information_schema.columns
WHERE table_schema = $1 AND table_name = $2
ORDER BY ordinal_position;

-- 读取表注释
SELECT obj_description(c.oid) AS table_comment
FROM pg_class c
JOIN pg_namespace n ON c.relnamespace = n.oid
WHERE n.nspname = $1 AND c.relname = $2;

-- 读取列注释
SELECT col_description(c.oid, a.attnum) AS column_comment
FROM pg_class c
JOIN pg_namespace n ON c.relnamespace = n.oid
JOIN pg_attribute a ON a.attrelid = c.oid
WHERE n.nspname = $1 AND c.relname = $2 AND a.attnum > 0 AND NOT a.attisdropped;
```

> **注意**：Rust 中使用 `$1, $2` 参数化查询（PostgreSQL 原生语法），而非 JDBC 的 `?`。

#### 3.2.4 模板引擎

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **HTML 模板** | Tera | 1.x | Jinja2/Django 风格语法，Rust 生态最流行的模板引擎 |
| **Markdown 生成** | 手动字符串拼接 | - | MVP 简单直接，无需额外依赖 |

**Tera 模板示例**：

```jinja2
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <title>{{ database_name }} - Schema Documentation</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
</head>
<body>
    <h1>Database: {{ database_name }}</h1>
    <p>Generated on: {{ generated_at }}</p>

    <h2>Tables</h2>
    {% for table in tables %}
    <h3 id="{{ table.name }}">{{ table.name }}</h3>
    {% if table.comment %}
    <p><strong>Description</strong>: {{ table.comment }}</p>
    {% endif %}
    <table>
        <tr><th>Column</th><th>Type</th><th>Nullable</th><th>Default</th><th>Comment</th></tr>
        {% for col in table.columns %}
        <tr>
            <td>{{ col.name }}</td>
            <td>{{ col.data_type }}</td>
            <td>{{ col.nullable }}</td>
            <td>{{ col.default_value | default(value="-") }}</td>
            <td>{{ col.comment | default(value="-") }}</td>
        </tr>
        {% endfor %}
    </table>
    {% endfor %}

    <h2>ER Diagram</h2>
    <div class="mermaid">{{ er_diagram }}</div>

    <script>mermaid.initialize({ startOnLoad: true });</script>
</body>
</html>
```

#### 3.2.5 序列化与 Diff

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **JSON 序列化** | serde + serde_json | 1.x | Rust 序列化标准，derive 宏零成本 |
| **YAML 配置** | serde_yaml | 0.9+ | 配置文件解析 |
| **Diff 算法** | similar | 2.x | 通用 diff 库，可用于文本和结构化数据对比 |

#### 3.2.6 日志与错误处理

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **日志框架** | tracing | 0.1+ | Rust 生态标准日志，支持结构化日志和 span |
| **日志输出** | tracing-subscriber | 0.3+ | 格式化输出 + 文件日志 |
| **错误类型** | anyhow | 1.x | 简化错误处理（应用层） |
| **自定义错误** | thiserror | 1.x | 定义精确错误类型（库层） |

#### 3.2.7 容器化工具

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **容器运行时** | Docker Engine | 24+ | 业界标准，testcontainers 依赖 |
| **容器编排** | Docker Compose | v2+ | 多服务编排（PostgreSQL + MySQL + MongoDB） |
| **容器化测试** | testcontainers-rs | 0.15+ | 自动拉起/销毁数据库容器，测试环境隔离 |

**为什么容器化**：
- 开发环境与 CI/CD 完全一致，消除"在我机器上能跑"问题
- testcontainers 每次测试用全新容器，测试之间零干扰
- 未来支持多数据库（MySQL、MongoDB）只需在 Docker Compose 中添加服务
- 本地不需要手动安装 PostgreSQL/MySQL/MongoDB，一条 `docker compose up` 即可

#### 3.2.8 测试

| 技术 | 选型 | 版本 | 选择理由 |
|------|------|------|----------|
| **单元测试** | 内置 `#[test]` | - | Rust 标准测试框架 |
| **异步测试** | tokio::test | - | 异步函数测试 |
| **容器化集成测试** | testcontainers-rs | 0.15+ | 自动启动/销毁 PostgreSQL/MySQL/MongoDB 容器 |
| **快照测试** | 内置 + assert_cmp | - | 确定性输入/输出对比 |
| **假数据** | fake / rstest | 2.x / 0.18+ | 参数化测试 + 假数据生成 |

### 3.3 技术选型总结

**Cargo.toml 核心依赖**：

```toml
[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }
indicatif = "0.17"
console = "0.15"
miette = { version = "5", features = ["fancy"] }
dialoguer = "0.11"           # 交互式提示（可选）

# 数据库
tokio = { version = "1", features = ["full"] }
tokio-postgres = "0.7"

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"

# 模板
tera = "1"

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 错误处理
anyhow = "1"
thiserror = "1"

# Diff
similar = "2"

# 工具
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }

[dev-dependencies]
testcontainers = "0.15"
testcontainers-modules = { version = "0.3", features = ["postgres", "mysql", "mongodb"] }
rstest = "0.18"
fake = { version = "2", features = ["derive"] }
```

**编译产物对比**：

| 维度 | Java (fat JAR) | Rust (静态链接) |
|------|----------------|-----------------|
| Linux x86_64 | ~50 MB + JRE | **~5 MB** |
| macOS aarch64 | ~50 MB + JRE | **~6 MB** |
| Windows x86_64 | ~50 MB + JRE | **~5 MB** |
| Docker 镜像 | ~250 MB | **~20 MB** |
| 启动时间 | ~1-2s | **< 50ms** |
| 内存占用 | ~100-300 MB | **~5-15 MB** |

---

## 四、开发环境搭建

### 4.1 环境概述

**开发模式**：Windows 物理机 + WSL Ubuntu + VSCode/CodeBuddy 远程连接

```
┌─────────────────────────────────────┐
│  Windows 11 物理机                  │
│  - VSCode (Remote - WSL)            │
│  - CodeBuddy (Remote - WSL)         │
│  - 浏览器（预览 HTML 输出）          │
├─────────────────────────────────────┤
│  WSL Ubuntu (开发环境)              │
│  - Rust toolchain (rustup)          │
│  - cargo + crates                   │
│  - Docker Engine + Docker Compose   │
│  - Git                              │
├─────────────────────────────────────┤
│  Docker 容器（按需启动）             │
│  - PostgreSQL 16 (开发/测试)        │
│  - MySQL 8 (未来扩展/测试)          │
│  - MongoDB 7 (未来扩展/测试)        │
└─────────────────────────────────────┘
```

**优势**：
- Linux 环境编译 Rust（原生支持最好、交叉编译最方便）
- Windows GUI（VSCode/浏览器）提供更好的交互体验
- **容器化数据库**：PostgreSQL/MySQL/MongoDB 全部通过 Docker 运行，与 CI/CD 环境完全一致
- **零安装依赖**：不需要在 WSL 中手动安装任何数据库服务，一条 `docker compose up` 即可
- 与生产环境一致（目标用户也在 Linux 服务器上使用）

### 4.2 WSL Ubuntu 环境初始化

#### 4.2.1 基础工具安装

```bash
# 更新系统
sudo apt update && sudo apt upgrade -y

# 基础开发工具
sudo apt install -y build-essential pkg-config libssl-dev git curl

# Git 配置
git config --global user.name "MasterHesse"
git config --global user.email "your-email@example.com"
```

#### 4.2.2 Rust 工具链安装

```bash
# 安装 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 选择默认安装（1. Stable）
source "$HOME/.cargo/env"

# 验证安装
rustc --version    # rustc 1.77.x
cargo --version    # cargo 1.77.x

# 常用组件
rustup component add clippy rustfmt rust-src

# 交叉编译目标（用于构建多平台 Release）
rustup target add x86_64-unknown-linux-musl   # 静态链接 Linux
rustup target add x86_64-pc-windows-gnu        # Windows 交叉编译
rustup target add aarch64-unknown-linux-gnu    # ARM Linux
```

#### 4.2.3 Docker 安装与容器化数据库

##### Docker Engine 安装

```bash
# 安装 Docker（使用官方 apt 源）
sudo apt install -y ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update && sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# 免 sudo 使用 Docker
sudo usermod -aG docker $USER
newgrp docker

# 验证安装
docker --version          # Docker version 24.x
docker compose version    # Docker Compose version v2.x
```

##### Docker Compose 编排（PostgreSQL + MySQL + MongoDB）

在项目根目录创建 `docker-compose.yml`，统一管理所有数据库容器：

```yaml
# docker-compose.yml
version: "3.8"

services:
  # PostgreSQL — MVP 主力数据库
  postgres:
    image: postgres:16-alpine
    container_name: crab-shell-postgres
    environment:
      POSTGRES_USER: hesse
      POSTGRES_PASSWORD: hesse
      POSTGRES_DB: crab_shell_test
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
      - ./docker/initdb:/docker-entrypoint-initdb.d  # 初始化 SQL
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U hesse"]
      interval: 5s
      timeout: 5s
      retries: 5

  # MySQL — v1.2 多数据库扩展
  mysql:
    image: mysql:8.0
    container_name: crab-shell-mysql
    environment:
      MYSQL_ROOT_PASSWORD: root
      MYSQL_USER: hesse
      MYSQL_PASSWORD: hesse
      MYSQL_DATABASE: crab_shell_test
    ports:
      - "3306:3306"
    volumes:
      - mysqldata:/var/lib/mysql
      - ./docker/initdb-mysql:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      interval: 5s
      timeout: 5s
      retries: 5
    profiles:
      - mysql  # 按需启动，不随默认 compose up 启动

  # MongoDB — v1.3 多数据库扩展
  mongodb:
    image: mongo:7
    container_name: crab-shell-mongodb
    environment:
      MONGO_INITDB_ROOT_USERNAME: hesse
      MONGO_INITDB_ROOT_PASSWORD: hesse
    ports:
      - "27017:27017"
    volumes:
      - mongodata:/data/db
    healthcheck:
      test: ["CMD", "mongosh", "--eval", "db.adminCommand('ping')"]
      interval: 5s
      timeout: 5s
      retries: 5
    profiles:
      - mongodb  # 按需启动

volumes:
  pgdata:
  mysqldata:
  mongodata:
```

**PostgreSQL 初始化数据**（`docker/initdb/01-schema.sql`）：

```sql
-- 测试表结构
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
COMMENT ON TABLE users IS '用户信息表';
COMMENT ON COLUMN users.id IS '用户ID';
COMMENT ON COLUMN users.username IS '用户名';

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    total_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
COMMENT ON TABLE orders IS '订单表';

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
```

##### 容器管理命令

```bash
# 启动 PostgreSQL（默认，用于日常开发和测试）
docker compose up -d postgres

# 启动 PostgreSQL + MySQL（测试多数据库支持时）
docker compose --profile mysql up -d

# 启动全部（含 MongoDB）
docker compose --profile mysql --profile mongodb up -d

# 查看容器状态
docker compose ps

# 查看日志
docker compose logs postgres -f

# 停止所有容器
docker compose down

# 停止并清除数据卷（完全重置）
docker compose down -v

# 连接 PostgreSQL（通过 Docker 网络）
docker compose exec postgres psql -U hesse -d crab_shell_test
```

#### 4.2.4 VSCode 远程连接配置

**VSCode 扩展安装**（在 Windows 端安装）：
1. `Remote - WSL`：远程连接 WSL
2. `rust-analyzer`：Rust 语言服务（IDE 支持）
3. `CodeLLDB`：Rust 调试
4. `Even Better TOML`：Cargo.toml 增强
5. `Error Lens`：内联显示错误

**推荐 VSCode 设置**（`.vscode/settings.json`）：

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

**推荐 VSCode 快捷键**：

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+Shift+B` | 编译项目 |
| `F5` | 调试运行 |
| `Ctrl+\`` | 切换终端 |
| `Ctrl+Shift+P` → "Rust Analyzer: Restart" | 重启语言服务 |

### 4.3 开发工作流

#### 4.3.1 日常开发流程

```bash
# 0. 启动数据库容器（首次或容器未运行时）
cd ~/projects/crab-shell
docker compose up -d postgres

# 1. 创建新功能分支（重要功能时）
git checkout -b feature/markdown-generator

# 2. 开发 + 实时编译检查
cargo check                  # 快速类型检查（< 5s）
cargo clippy                 # Lint 检查
cargo fmt                    # 格式化代码

# 3. 运行测试（testcontainers 自动管理独立容器）
cargo test                   # 运行所有测试（含容器化集成测试）
cargo test -- --nocapture    # 运行测试并显示输出

# 4. 运行 CLI（连接 Docker 中的 PostgreSQL）
cargo run -- generate --url postgres://hesse:hesse@localhost:5432/crab_shell_test --output ./test-output

# 5. 构建优化版本
cargo build --release        # 优化编译（~30s - 2min）

# 6. 提交
git add -A
git commit -m "feat: add markdown data dictionary generator"
```

#### 4.3.2 预提交检查（使用 cargo-hack 或 just）

推荐安装 `just` 作为任务运行器：

```bash
cargo install just
```

`justfile`：

```just
# 默认任务：完整检查
default: check test lint fmt-check

# 快速类型检查
check:
    cargo check

# 运行测试（testcontainers 自动管理数据库容器）
test:
    cargo test

# Lint 检查
lint:
    cargo clippy -- -D warnings

# 格式检查（不自动修改）
fmt-check:
    cargo fmt -- --check

# 自动格式化
fmt:
    cargo fmt

# 完整发布构建
release:
    cargo build --release

# 运行 CLI（开发模式）
run *ARGS:
    cargo run -- {{ARGS}}

# 启动开发数据库容器
db-up:
    docker compose up -d postgres

# 启动全部数据库（含 MySQL + MongoDB）
db-up-all:
    docker compose --profile mysql --profile mongodb up -d

# 停止所有数据库容器
db-down:
    docker compose down

# 查看数据库容器状态
db-status:
    docker compose ps

# 运行单个容器化集成测试（可观测 Docker 日志）
integration-test:
    RUST_LOG=info cargo test --test integration -- --nocapture
```

### 4.4 项目目录结构

```
crab-shell/
├── Cargo.toml                  # 项目配置和依赖
├── Cargo.lock                  # 锁定依赖版本
├── rustfmt.toml                # Rust 格式化配置
├── clippy.toml                 # Clippy Lint 配置
├── justfile                    # 任务运行器
├── docker-compose.yml          # 容器编排（PostgreSQL/MySQL/MongoDB）
├── .dockerignore               # Docker 构建排除
├── Dockerfile                  # 多阶段构建（Release 镜像）
├── .github/
│   └── workflows/
│       ├── ci.yml              # CI: test + clippy + fmt（service containers）
│       └── release.yml         # Release: 交叉编译 + GitHub Release
├── src/
│   ├── main.rs                 # 入口（clap CLI 定义）
│   │
│   │   # ===== 应用层 (Application) =====
│   ├── cli/
│   │   ├── mod.rs
│   │   └── commands.rs         # generate, snapshot, diff 子命令
│   ├── config.rs               # 配置文件解析
│   │
│   │   # ===== 领域层 (Domain) =====
│   ├── schema/                 # Schema 领域 — 数据模型
│   │   ├── mod.rs
│   │   ├── database.rs         # DatabaseSchema struct
│   │   ├── table.rs            # Table, Column, Constraint, Index
│   │   └── traits.rs           # SchemaReader trait（领域接口）
│   ├── generator/              # Generator 领域 — 文档生成
│   │   ├── mod.rs
│   │   ├── traits.rs           # DocumentGenerator trait（领域接口）
│   │   ├── markdown.rs         # Markdown 文档生成
│   │   ├── mermaid.rs          # Mermaid ER 图生成
│   │   └── html.rs             # HTML 文档生成（v1.0）
│   ├── snapshot/               # Snapshot 领域 — 快照管理
│   │   ├── mod.rs
│   │   ├── store.rs            # 快照保存/加载
│   │   └── differ.rs           # Schema Diff 引擎
│   │
│   │   # ===== 基础设施层 (Infrastructure) =====
│   ├── infra/
│   │   ├── mod.rs
│   │   ├── postgres.rs         # PostgreSQL 适配器（实现 SchemaReader）
│   │   └── connection.rs       # 连接池管理
│   │
│   └── error.rs                # 错误类型定义
├── tests/
│   ├── integration.rs          # 容器化集成测试（testcontainers）
│   └── common/
│       └── mod.rs              # 测试辅助函数
├── docker/
│   ├── initdb/                 # PostgreSQL 初始化 SQL
│   │   └── 01-schema.sql
│   ├── initdb-mysql/           # MySQL 初始化 SQL
│   │   └── 01-schema.sql
│   └── initdb-mongodb/         # MongoDB 初始化脚本
│       └── 01-schema.js
├── templates/
│   ├── default.html            # 默认 HTML 模板（v1.0）
│   └── default.css             # 样式文件
├── docs/
│   └── examples/               # 示例输出文件
├── README.md                   # 项目说明
├── CHANGELOG.md                # 变更日志
└── LICENSE                     # 开源协议
```

**DDD 分层说明**：
- **应用层** (`cli/`, `config.rs`)：CLI 入口、配置解析、任务编排，不含业务逻辑
- **领域层** (`schema/`, `generator/`, `snapshot/`)：核心业务逻辑，通过 trait 定义接口，不依赖具体数据库实现
- **基础设施层** (`infra/`)：实现领域层定义的 trait（如 `SchemaReader`），负责与外部系统（PostgreSQL）交互

---

## 五、系统架构设计

### 5.1 架构原则

1. **DDD 分层架构**：应用层（CLI/配置）→ 领域层（Schema/Generator/Snapshot）→ 基础设施层（数据库适配器），依赖方向严格单向
2. **模块化**：按领域（Domain）划分 crate module，高内聚低耦合；每个领域定义自己的 trait 接口
3. **异步优先**：数据库 I/O 使用 async，其他模块可按需选择
4. **trait 抽象**：领域层通过 trait 定义接口，基础设施层实现（如 `SchemaReader` trait），未来新增数据库类型只需新增适配器
5. **零配置启动**：MVP 阶段支持环境变量直接运行
6. **容器化一致**：开发/测试/CI 使用相同的 Docker 容器环境

### 5.2 系统架构图

#### 5.2.1 高层架构（DDD 分层 + 容器化）

```
┌─────────────┐     ┌──────────────────────────────────────────────────┐     ┌──────────────┐
│             │     │             crab-shell (Rust binary)              │     │              │
│   用户终端   │────▶│                                                  │────▶│  文件系统     │
│  (CLI/TUI)  │     │  ┌─ 应用层 (Application) ────────────────────┐   │     │  .md .html   │
│             │     │  │  CLI (clap) → Config → Task Orchestrator  │   │     │  .json       │
└─────────────┘     │  └──────────────────┬───────────────────────┘   │     └──────────────┘
                    │                     │                            │
                    │  ┌─ 领域层 (Domain) ─┼────────────────────────┐   │
                    │  │                   ▼                        │   │
                    │  │  ┌────────────┐ ┌────────────┐ ┌────────┐ │   │
                    │  │  │ Schema 领域 │ │ Generator  │ │Snapshot│ │   │
                    │  │  │ (数据模型)  │ │  领域      │ │ 领域   │ │   │
                    │  │  │ SchemaReader│ │DocGenerator│ │ Differ │ │   │
                    │  │  │  (trait)   │ │  (trait)   │ │        │ │   │
                    │  │  └─────┬──────┘ └────────────┘ └────────┘ │   │
                    │  └────────┼───────────────────────────────────┘   │
                    │           │ 依赖方向：领域层 ← 基础设施层            │
                    │  ┌─ 基础设施层 ─┼───────────────────────────────┐   │
                    │  │  (Infrastructure) ▼                        │   │
                    │  │  ┌──────────────┐ ┌────────┐ ┌───────────┐ │   │
                    │  │  │ PostgreSQL   │ │ MySQL  │ │ MongoDB   │ │   │
                    │  │  │ Adapter      │ │Adapter │ │ Adapter   │ │   │
                    │  │  │(impl SchemaR)│ │(未来)  │ │ (未来)    │ │   │
                    │  │  └──────────────┘ └────────┘ └───────────┘ │   │
                    │  └───────────────────────────────────────────┘   │
                    └──────────────────┬───────────────────────────────┘
                                       ▼
                    ┌──────────────────────────────────────────────────┐
                    │  Docker Compose 容器（开发/测试/CI 统一环境）      │
                    │  ┌──────────────┐ ┌────────┐ ┌───────────┐     │
                    │  │ PostgreSQL 16│ │ MySQL 8│ │ MongoDB 7 │     │
                    │  │  :5432       │ │ :3306  │ │  :27017   │     │
                    │  └──────────────┘ └────────┘ └───────────┘     │
                    └──────────────────────────────────────────────────┘
```

#### 5.2.2 模块依赖关系（DDD 分层）

```
main.rs
  │
  └── cli (clap 命令定义) ─── 应用层
        └── config (配置解析)
        └── TaskOrchestrator (任务编排)
              │
              ├── schema/ ────────────── 领域层
              │     ├── database.rs (DatabaseSchema struct)
              │     ├── table.rs    (Table, Column, Constraint, Index)
              │     └── traits.rs   (SchemaReader trait ← 接口)
              │
              ├── generator/ ─────────── 领域层
              │     ├── traits.rs   (DocumentGenerator trait ← 接口)
              │     ├── markdown.rs (impl DocumentGenerator)
              │     ├── mermaid.rs  (impl DocumentGenerator)
              │     └── html.rs     (impl DocumentGenerator, v1.0)
              │
              ├── snapshot/ ──────────── 领域层
              │     ├── store.rs    (快照保存/加载)
              │     └── differ.rs   (Schema Diff 引擎)
              │
              └── infra/ ─────────────── 基础设施层
                    ├── postgres.rs  (impl SchemaReader for PostgresAdapter)
                    ├── mysql.rs     (impl SchemaReader for MySqlAdapter, 未来)
                    ├── mongodb.rs   (impl SchemaReader for MongoAdapter, 未来)
                    └── connection.rs(连接池管理)

依赖规则：
  - 应用层 → 领域层 (单向依赖)
  - 领域层 → 领域层 (同层可互相引用，但建议最小化)
  - 基础设施层 → 领域层 (实现领域层定义的 trait)
  - 领域层 ←─╳── 基础设施层 (领域层不依赖基础设施层！)
```

### 5.3 核心数据模型

```rust
// src/schema/database.rs — Schema 领域模型

use serde::{Deserialize, Serialize};

/// 完整的数据库 Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub database: String,
    pub schema: String,
    pub tables: Vec<Table>,
    pub views: Vec<View>,
    pub enums: Vec<EnumType>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub comment: Option<String>,
    pub columns: Vec<Column>,
    pub primary_key: Option<PrimaryKey>,
    pub foreign_keys: Vec<ForeignKey>,
    pub indexes: Vec<Index>,
    pub constraints: Vec<Constraint>,
}

/// 列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub full_data_type: String,       // 如 "character varying(255)"
    pub nullable: bool,
    pub default_value: Option<String>,
    pub comment: Option<String>,
    pub ordinal_position: i32,
    pub is_identity: bool,            // 是否 SERIAL/IDENTITY
}

/// 主键
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryKey {
    pub name: String,
    pub columns: Vec<String>,
}

/// 外键
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKey {
    pub name: String,
    pub columns: Vec<String>,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
    pub on_update: Option<String>,
    pub on_delete: Option<String>,
}

/// 索引
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub index_type: Option<String>,   // btree, hash, gin, gist...
}

/// 约束
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub constraint_type: String,      // UNIQUE, CHECK, EXCLUSION
    pub definition: Option<String>,
}
```

### 5.4 核心 Trait 设计

```rust
// src/generator/traits.rs

/// 文档生成器 trait——不同输出格式实现此 trait
pub trait DocumentGenerator {
    /// 生成文档内容
    fn generate(&self, schema: &DatabaseSchema) -> anyhow::Result<String>;
    /// 获取输出文件扩展名
    fn file_extension(&self) -> &str;
}

/// Schema 快照 trait
pub trait SnapshotManager {
    /// 保存快照到文件
    fn save(&self, schema: &DatabaseSchema, path: &Path) -> anyhow::Result<()>;
    /// 从文件加载快照
    fn load(&self, path: &Path) -> anyhow::Result<DatabaseSchema>;
}
```

### 5.5 数据流设计

```
用户执行: crab-shell generate --url postgres://... --output ./docs
    │
    ▼
┌─ main.rs ──────────────────────────────────────────────┐
│ clap 解析参数 → Config struct                           │
└──────────────────────────┬─────────────────────────────┘
                           ▼
┌─ db/connection.rs ─────────────────────────────────────┐
│ 连接 PostgreSQL（异步，带重试）                          │
└──────────────────────────┬─────────────────────────────┘
                           ▼
┌─ db/querier.rs ────────────────────────────────────────┐
│ 异步读取 Schema:                                        │
│   1. 读取表清单 (information_schema.tables)              │
│   2. 并行读取每张表的列、约束、索引                       │
│   3. 读取注释 (pg_description)                          │
│   → DatabaseSchema struct                               │
└──────────────────────────┬─────────────────────────────┘
                           ▼
┌─ generator/ ───────────────────────────────────────────┐
│ 并行生成（tokio::join!）:                                │
│   1. MarkdownGenerator → String (Markdown 文档)         │
│   2. MermaidRenderer → String (Mermaid ER 图)           │
│   3. JSON 序列化 → String (JSON 快照)                   │
└──────────────────────────┬─────────────────────────────┘
                           ▼
┌─ 文件写入 ─────────────────────────────────────────────┐
│ 写入: ./docs/mydb-schema-20260513.md                    │
│ 写入: ./docs/mydb-snapshot-20260513.json                │
└──────────────────────────┬─────────────────────────────┘
                           ▼
                    终端输出成功消息 + 进度/耗时
```

### 5.6 错误处理设计

```rust
// src/error.rs

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum SchemaDocError {
    #[error("数据库连接失败: {host}:{port} - {reason}")]
    #[diagnostic(code(crab_shell::db_connection))]
    ConnectionFailed {
        host: String,
        port: u16,
        reason: String,
        #[help]
        help: "请检查数据库是否在运行，连接参数是否正确。\
               \n也可以通过环境变量配置: PGHOST, PGPORT, PGDATABASE, PGUSER, PGPASSWORD",
    },

    #[error("数据库权限不足: {0}")]
    #[diagnostic(code(crab_shell::permission))]
    PermissionDenied(String),

    #[error("Schema '{0}' 不存在")]
    #[diagnostic(code(crab_shell::schema_not_found))]
    SchemaNotFound(String),

    #[error("无法写入输出目录: {path} - {reason}")]
    #[diagnostic(code(crab_shell::io_error))]
    OutputWriteFailed {
        path: String,
        reason: String,
    },

    #[error("配置文件格式错误: {0}")]
    #[diagnostic(code(crab_shell::config_error))]
    ConfigError(String),

    #[error("快照加载失败: {0}")]
    #[diagnostic(code(crab_shell::snapshot_error))]
    SnapshotError(String),
}

// CLI 层使用 anyhow 简化错误传播
// 库层使用 thiserror 定义精确错误类型
// 展示层使用 miette 提供漂亮的错误报告
```

**用户看到的错误示例**：

```
Error: 数据库连接失败: localhost:5432 - connection refused

  × crab_shell::db_connection
    ╭────
    │
    │   3 │     --url postgres://localhost:5432/mydb
    │     ·            ~~~~~~~~~~~~~~~~~~~~~~~~~
    │     ╰── connection refused
    │
    │ 帮助: 请检查数据库是否在运行，连接参数是否正确。
    │       也可以通过环境变量配置: PGHOST, PGPORT, PGDATABASE, PGUSER, PGPASSWORD
    ╰────
```

---

## 六、功能设计

### 6.1 MVP 功能清单

| 功能模块 | 功能点 | 优先级 | 描述 |
|----------|--------|--------|------|
| **数据库连接** | tokio-postgres 异步连接 | P0 | 支持连接字符串或环境变量 |
| **数据库连接** | 连接重试 + 超时 | P0 | 默认重试 3 次，超时 30s |
| **数据库连接** | 只读权限验证 | P1 | 警告但不阻止（有些查询需要 pg_catalog） |
| **Schema 读取** | 读取表/列/约束/索引/注释 | P0 | information_schema + pg_catalog |
| **Schema 读取** | 读取视图 | P1 | information_schema.views |
| **Schema 读取** | 读取枚举类型 | P2 | pg_type + pg_enum |
| **文档生成** | Markdown 数据字典 | P0 | 表格形式，含列详情和注释 |
| **文档生成** | Mermaid ER 图 | P0 | 含表关系和基数 |
| **文档生成** | 按 schema 分组 | P1 | 多 schema 时分组渲染 |
| **快照管理** | JSON 快照生成/加载 | P0 | serde 序列化 |
| **CLI** | `crab-shell generate` | P0 | 主命令 |
| **CLI** | `--help` / `--version` | P0 | clap 自动生成 |
| **CLI** | 彩色输出 + 进度条 | P1 | indicatif + console |
| **CLI** | 友好错误提示 | P1 | miette |

### 6.2 CLI 命令设计

```rust
// src/main.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "crab-shell")]
#[command(version, about = "PostgreSQL Schema 文档生成器")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// 详细日志模式 (-vv 更详细)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// 生成数据库文档
    Generate {
        /// 数据库连接字符串
        #[arg(short, long, env = "DATABASE_URL")]
        url: Option<String>,

        /// 目标 schema 名称
        #[arg(short = 's', long, default_value = "public")]
        schema: String,

        /// 输出目录
        #[arg(short, long, default_value = "./docs")]
        output: String,

        /// 输出格式
        #[arg(short, long, default_value = "markdown", value_enum)]
        format: OutputFormat,

        /// 配置文件路径
        #[arg(short, long)]
        config: Option<String>,

        /// 包含视图
        #[arg(long)]
        include_views: bool,

        /// 排除的表（逗号分隔）
        #[arg(long)]
        exclude_tables: Option<String>,

        /// 每个图最大表数（超过则分组）
        #[arg(long, default_value = "30")]
        max_tables_per_diagram: usize,
    },

    /// Schema 快照管理
    Snapshot {
        #[command(subcommand)]
        action: SnapshotAction,
    },

    /// 对比两个快照
    Diff {
        /// 旧快照文件路径
        old: String,

        /// 新快照文件路径
        new: String,

        /// 输出文件路径
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum SnapshotAction {
    /// 创建新快照
    Take {
        #[arg(short, long, env = "DATABASE_URL")]
        url: Option<String>,
        #[arg(short, long, default_value = "public")]
        schema: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// 查看快照信息
    Info {
        snapshot: String,
    },
    /// 列出所有快照
    List {
        #[arg(short, long, default_value = "./snapshots")]
        dir: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Markdown,
    Html,   // v1.0
    Json,   // 快照模式
    All,
}
```

**使用示例**：

```bash
# 基本用法（使用环境变量）
export DATABASE_URL="postgres://hesse:hesse@localhost:5432/crab_shell_test"
crab-shell generate --output ./docs

# 指定连接字符串
crab-shell generate --url "postgres://user:pass@host:5432/mydb" --output ./docs

# 使用配置文件
crab-shell generate --config crab-shell.yaml

# 只生成指定 schema
crab-shell generate --schema public --output ./docs

# 排除某些表
crab-shell generate --exclude-tables "temp_*,old_logs" --output ./docs

# 生成快照
crab-shell snapshot take --output ./snapshots/mydb-20260513.json

# 对比快照
crab-shell diff ./snapshots/mydb-20260510.json ./snapshots/mydb-20260513.json
```

### 6.3 配置文件

**crab-shell.yaml**：

```yaml
database:
  # 连接字符串（优先级低于命令行参数和环境变量）
  url: "postgres://localhost:5432/mydb"
  # 或分开指定
  host: localhost
  port: 5432
  database: mydb
  username: myuser
  # 密码支持环境变量引用
  password: "${PGPASSWORD}"
  schema: public
  # 连接超时（秒）
  connect_timeout: 30
  # 连接重试次数
  max_retries: 3

output:
  format: markdown
  path: "./docs"
  # 文件名模板（支持变量）
  filename_template: "{database}-schema-{date}.md"

options:
  include_views: true
  include_enums: false
  include_comments: true
  exclude_tables: []
  exclude_schemas: []
  max_tables_per_diagram: 30
  # Mermaid ER 图配置
  mermaid:
    # 是否在 Markdown 中嵌入 ER 图
    embed_in_markdown: true
    # 关系线样式
    relationship_style: "simple"  # simple | detailed
```

### 6.4 Markdown 输出示例

```markdown
# crab_shell_test - Database Schema

> Generated by crab-shell v0.1.0 on 2026-05-13 12:00:00

## Table of Contents

- [users](#users) (4 columns, 1 index)
- [orders](#5 columns, 2 indexes)

---

## users

> 用户信息表

| # | Column | Type | Nullable | Default | Comment |
|---|--------|------|----------|---------|---------|
| 1 | id | INTEGER | NO | nextval('users_id_seq') | 用户ID |
| 2 | username | VARCHAR(50) | NO | | 用户名 |
| 3 | email | VARCHAR(100) | NO | | 邮箱 |
| 4 | created_at | TIMESTAMP | YES | CURRENT_TIMESTAMP | |

**Primary Key**: `id` (users_pkey)

**Indexes**:
| Name | Columns | Unique | Type |
|------|---------|--------|------|
| users_email_key | email | YES | btree |

---

## orders

> 订单表

| # | Column | Type | Nullable | Default | Comment |
|---|--------|------|----------|---------|---------|
| 1 | id | INTEGER | NO | nextval('orders_id_seq') | |
| 2 | user_id | INTEGER | NO | | |
| 3 | total_amount | DECIMAL(10,2) | NO | | |
| 4 | status | VARCHAR(20) | NO | 'PENDING' | |
| 5 | created_at | TIMESTAMP | YES | CURRENT_TIMESTAMP | |

**Primary Key**: `id` (orders_pkey)

**Foreign Keys**:
| Name | Column(s) | References |
|------|-----------|------------|
| orders_user_id_fkey | user_id | users(id) ON DELETE NO ACTION |

**Indexes**:
| Name | Columns | Unique | Type |
|------|---------|--------|------|
| idx_orders_user_id | user_id | NO | btree |
| idx_orders_status | status | NO | btree |

---

## ER Diagram

```mermaid
erDiagram
    users ||--o{ orders : "orders_user_id_fkey"
    users {
        int id PK
        string username
        string email
        timestamp created_at
    }
    orders {
        int id PK
        int user_id FK
        decimal total_amount
        string status
        timestamp created_at
    }
```
```

---

## 七、非功能性设计

### 7.1 性能需求

| 指标 | 目标值 | 说明 |
|------|--------|------|
| **冷启动时间** | < 100ms | 单二进制，无 JVM 预热 |
| **文档生成时间** | < 30 秒（< 50 表） | 异步并行读取 + 生成 |
| **内存占用** | < 50 MB | Rust 零运行时开销 |
| **二进制体积** | < 10 MB | 静态链接，musl target |
| **JSON 快照大小** | < 5 MB（< 100 表） | serde 紧凑序列化 |

### 7.2 安全性需求

| 需求 | 实现策略 |
|------|----------|
| **只读访问** | 代码中只执行 SELECT 查询 |
| **连接信息安全** | 优先环境变量，配置文件支持 `${VAR}` 引用 |
| **SQL 注入防护** | tokio-postgres 参数化查询（`$1, $2`），杜绝字符串拼接 |
| **密码不落盘日志** | tracing 过滤敏感信息，ConnectionConfig Debug impl 隐藏密码 |
| **pgpass 支持** | 支持 `~/.pgpass` 文件（PostgreSQL 标准） |

### 7.3 兼容性需求

| 需求 | 实现策略 |
|------|----------|
| **PostgreSQL 12+** | 使用 `information_schema`（标准接口）+ `pg_catalog`（增强） |
| **Linux / macOS / Windows** | 静态编译，零系统依赖 |
| **x86_64 + aarch64** | 交叉编译 + CI 多目标构建 |
| **GFM 兼容** | Markdown 输出遵循 GitHub Flavored Markdown |

### 7.4 可维护性

| 实践 | 工具 |
|------|------|
| **代码格式** | `cargo fmt`（rustfmt） |
| **Lint** | `cargo clippy`（禁用所有 warning） |
| **单元测试** | `#[test]` + `#[tokio::test]` |
| **集成测试** | testcontainers-rs（自动启动 PostgreSQL） |
| **CI** | GitHub Actions（test + clippy + fmt + 交叉编译） |
| **文档** | `///` doc comment + README |

---

## 八、CLI/TUI 交互设计

### 8.1 CLI 体验设计

**设计目标**：让 CLI 用起来"感觉很快、很现代"。

#### 8.1.1 正常输出（带进度和颜色）

```
$ crab-shell generate --url postgres://localhost:5432/mydb

  crab-shell v0.1.0

  Connecting to postgresql://localhost:5432/mydb... done (0.3s)
  Reading schema 'public'...

  ✓ users (4 columns, 1 index, 1 pk)
  ✓ orders (5 columns, 2 indexes, 1 pk, 1 fk)
  ✓ order_items (6 columns, 2 indexes, 2 fks)
  ✓ products (8 columns, 3 indexes, 1 pk)
  ✓ categories (3 columns, 1 pk)

  Generating documentation...
  ✓ Markdown data dictionary  (0.1s)
  ✓ Mermaid ER diagram         (0.0s)
  ✓ JSON snapshot              (0.0s)

  Output:
    ./docs/mydb-schema-20260513.md    (2.4 KB)
    ./docs/mydb-snapshot-20260513.json (1.8 KB)

  Done in 0.8s
```

#### 8.1.2 错误输出（miette 漂亮报告）

```
$ crab-shell generate --url postgres://localhost:5432/nonexistent

  Error

    × 数据库连接失败: localhost:5432/nonexistent
    │   cause: database "nonexistent" does not exist

    help: 请检查数据库名称是否正确。
          当前连接目标: postgresql://localhost:5432/nonexistent
          可用数据库: postgres, template0, template1, crab_shell_test
```

#### 8.1.3 帮助输出（clap 自动生成）

```
$ crab-shell generate --help

Generate database documentation from a PostgreSQL schema

Usage: crab-shell generate [OPTIONS]

Options:
  -u, --url <URL>              Database connection string [env: DATABASE_URL]
  -s, --schema <SCHEMA>        Target schema name [default: public]
  -o, --output <DIR>           Output directory [default: ./docs]
  -f, --format <FORMAT>        Output format [default: markdown] [possible values: markdown, html, json, all]
  -c, --config <FILE>          Configuration file path
      --include-views          Include views in documentation
      --exclude-tables <LIST>  Tables to exclude (comma-separated, glob supported)
      --max-tables <N>         Max tables per ER diagram [default: 30]
  -h, --help                   Print help
  -V, --version                Print version
```

### 8.2 TUI 进阶规划（v2.0）

> Rust 的 TUI 生态（ratatui + crossterm）是其相比 Java 的独特优势。可以在 CLI 稳定后自然进阶。

**演进路径**：

```
CLI (clap + indicatif)
  ↓ 添加交互式选择
CLI + inquire（交互式提示）
  ↓ 添加终端 UI
TUI (ratatui + crossterm)
  ↓ 添加 Web 服务
Web UI (axum + React)
```

**TUI 原型设计**：

```
┌─ crab-shell ─────────────────────────────────────────────┐
│                                                           │
│  Database: mydb (PostgreSQL 15.3)                        │
│  Schema: public (15 tables, 3 views)                     │
│                                                           │
│  ┌──────────┬───────────────────────────────────────────┐ │
│  │ ► Tables │  users                                    │ │
│  │   Views  │  ───────────────────────────────────────  │ │
│  │   Enums  │  用户信息表                                │ │
│  │          │                                           │ │
│  │          │  Columns:                                  │ │
│  │          │  ┌──────────┬──────────┬───────┬────────┐ │ │
│  │          │  │ Column   │ Type     │ Null  │ Comment│ │ │
│  │          │  ├──────────┼──────────┼───────┼────────┤ │ │
│  │          │  │ id       │ INTEGER  │ NO    │ PK     │ │ │
│  │          │  │ username │ VARCHAR  │ NO    │ 用户名 │ │ │
│  │          │  │ email    │ VARCHAR  │ NO    │ 邮箱   │ │ │
│  │          │  └──────────┴──────────┴───────┴────────┘ │ │
│  │          │                                           │ │
│  │          │  [g] Generate  [d] Diff  [q] Quit         │ │
│  └──────────┴───────────────────────────────────────────┘ │
│                                                           │
│  ↑↓ Navigate  Enter Select  g Generate  q Quit           │
└───────────────────────────────────────────────────────────┘
```

**TUI 技术选型**：

| crate | 用途 |
|-------|------|
| `ratatui` | 终端 UI 框架（Widget 渲染） |
| `crossterm` | 跨平台终端控制（颜色、光标、事件） |
| `tui-textarea` | 文本输入组件 |

---

## 九、测试策略

### 9.1 TDD + 容器化测试理念

本项目采用 **TDD（测试驱动开发）** 作为核心开发方法，配合 **容器化测试基础设施**，确保代码质量和环境一致性。

**核心理念**：
- **先写测试，后写代码**：测试定义行为契约（"它应该做什么"），实现代码只负责满足契约
- **黑箱思维**：测试从使用者视角编写，不关心内部实现细节，只关心输入和输出
- **容器化隔离**：所有涉及外部服务的测试使用 testcontainers 自动管理 Docker 容器
- **开发/CI 零差异**：本地开发和 CI 使用完全相同的容器化测试环境

### 9.2 测试分层（测试金字塔）

```
              ┌────────┐
              │  E2E   │   手动测试（真实环境验证）
             ╱  (少量)  ╲
            ╱────────────╲
           │  集成测试    │   testcontainers 容器化
           │  (适量)      │   PostgreSQL/MySQL/MongoDB
          ╱──────────────╲
         │  单元测试       │   #[test] + #[tokio::test]
         │  (大量，核心层)  │   纯逻辑，无外部依赖
        ╱──────────────────╲
       │  快照测试           │   确定性输入/输出对比
       └────────────────────┘

各层比例：单元 70% / 集成 20% / 快照 8% / E2E 2%
```

**测试分层原则**：
- **单元测试**（70%）：覆盖核心领域逻辑（Schema 模型、Diff 引擎、文档生成），纯函数，无 I/O
- **容器化集成测试**（20%）：验证与真实数据库的交互，使用 testcontainers 自动管理
- **快照测试**（8%）：确保输出稳定性，防止意外修改
- **E2E 测试**（2%）：手动验证完整流程

### 9.3 TDD 实践规范

#### 9.3.1 测试命名约定（行为驱动）

```rust
// ✅ 正确：描述行为，读测试即知需求
#[test]
fn should_generate_table_of_contents_for_multiple_tables() { ... }

#[test]
fn should_include_column_comments_in_markdown_output() { ... }

#[test]
fn should_detect_new_column_added_in_diff() { ... }

#[test]
fn should_return_empty_tables_list_for_empty_schema() { ... }

// ❌ 错误：描述实现细节
#[test]
fn test_markdown_generator() { ... }
#[test]
fn test_loop_through_tables() { ... }
```

#### 9.3.2 TDD 工作流（Red → Green → Refactor）

每个功能模块严格遵循 TDD 三步循环：

| 步骤 | 动作 | 示例（以 Markdown 生成器为例） |
|------|------|-------------------------------|
| **Red** | 编写失败测试 | `should_generate_column_table_headers` → 运行，编译失败 |
| **Green** | 最小实现 | 写 `generate()` 方法骨架，输出硬编码表头 → 测试通过 |
| **Refactor** | 重构优化 | 提取表头模板，支持动态列 → 测试仍通过 |

### 9.4 单元测试（核心领域）

**覆盖重点**：Schema 领域模型、Generator 领域、Snapshot/Diff 领域。
**原则**：纯逻辑测试，**不连接任何外部服务**。

```rust
// src/generator/markdown.rs — Generator 领域单元测试

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_schema() -> DatabaseSchema {
        DatabaseSchema {
            database: "testdb".into(),
            schema: "public".into(),
            tables: vec![Table {
                name: "users".into(),
                comment: Some("用户信息表".into()),
                columns: vec![
                    Column {
                        name: "id".into(),
                        data_type: "integer".into(),
                        full_data_type: "integer".into(),
                        nullable: false,
                        default_value: Some("nextval('users_id_seq')".into()),
                        comment: Some("用户ID".into()),
                        ordinal_position: 1,
                        is_identity: true,
                    },
                    Column {
                        name: "username".into(),
                        data_type: "character varying".into(),
                        full_data_type: "character varying(50)".into(),
                        nullable: false,
                        default_value: None,
                        comment: Some("用户名".into()),
                        ordinal_position: 2,
                        is_identity: false,
                    },
                ],
                primary_key: Some(PrimaryKey {
                    name: "users_pkey".into(),
                    columns: vec!["id".into()],
                }),
                foreign_keys: vec![],
                indexes: vec![],
                constraints: vec![],
            }],
            views: vec![],
            enums: vec![],
            generated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn should_generate_table_name_heading() {
        let schema = sample_schema();
        let gen = MarkdownGenerator::new();
        let md = gen.generate(&schema).unwrap();
        assert!(md.contains("## users"));
    }

    #[test]
    fn should_include_table_comment() {
        let schema = sample_schema();
        let gen = MarkdownGenerator::new();
        let md = gen.generate(&schema).unwrap();
        assert!(md.contains("用户信息表"));
    }

    #[test]
    fn should_generate_column_table_headers() {
        let schema = sample_schema();
        let gen = MarkdownGenerator::new();
        let md = gen.generate(&schema).unwrap();
        assert!(md.contains("| Column | Type | Nullable |"));
    }

    #[test]
    fn should_embed_mermaid_er_diagram() {
        let schema = sample_schema();
        let gen = MarkdownGenerator::new();
        let md = gen.generate(&schema).unwrap();
        assert!(md.contains("```mermaid"));
        assert!(md.contains("erDiagram"));
    }

    #[test]
    fn should_handle_empty_schema_gracefully() {
        let schema = DatabaseSchema {
            database: "empty_db".into(),
            schema: "public".into(),
            tables: vec![],
            views: vec![],
            enums: vec![],
            generated_at: chrono::Utc::now(),
        };
        let gen = MarkdownGenerator::new();
        let md = gen.generate(&schema).unwrap();
        assert!(md.contains("empty_db"));
        assert!(md.contains("0 tables"));
    }
}
```

### 9.5 容器化集成测试（testcontainers）

**所有涉及数据库的集成测试使用 testcontainers**，自动拉起/销毁 Docker 容器，无需本地安装任何数据库。

#### 9.5.1 PostgreSQL 容器化集成测试

```rust
// tests/integration.rs

use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

#[tokio::test]
async fn should_read_schema_from_postgres_container() {
    // testcontainers 自动拉起 PostgreSQL Docker 容器
    let postgres = Postgres::default().start().await.unwrap();
    let conn_str = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        postgres.get_host().await.unwrap(),
        postgres.get_host_port_ipv4(5432).await.unwrap()
    );

    // 在容器中创建测试表
    let (client, connection) = tokio_postgres::connect(&conn_str, tokio_postgres::NoTls)
        .await
        .unwrap();
    tokio::spawn(async move { if let Err(e) = connection.await { eprintln!("DB error: {e}"); } });

    client
        .batch_execute(
            "CREATE TABLE test_users (
                id SERIAL PRIMARY KEY,
                name VARCHAR(50) NOT NULL
            );
            COMMENT ON TABLE test_users IS '测试用户表';"
        )
        .await
        .unwrap();

    // 使用 SchemaReader trait 读取（通过基础设施层适配器）
    let reader = PostgresAdapter::new(&conn_str).await.unwrap();
    let schema = reader.read_schema("public").await.unwrap();

    assert_eq!(schema.tables.len(), 1);
    assert_eq!(schema.tables[0].name, "test_users");
    assert_eq!(schema.tables[0].comment, Some("测试用户表".into()));
}
```

#### 9.5.2 MySQL 容器化集成测试（v1.2 扩展）

```rust
// tests/integration_mysql.rs — 未来扩展

use testcontainers::runners::AsyncRunner;
use testcontainers_modules::mysql::Mysql;

#[tokio::test]
async fn should_read_schema_from_mysql_container() {
    // testcontainers 自动拉起 MySQL Docker 容器
    let mysql = Mysql::default().start().await.unwrap();
    let conn_str = format!(
        "mysql://root:root@{}:{}/mysql",
        mysql.get_host().await.unwrap(),
        mysql.get_host_port_ipv4(3306).await.unwrap()
    );

    // 使用 MySQL 适配器读取
    let reader = MySqlAdapter::new(&conn_str).await.unwrap();
    let schema = reader.read_schema("test_db").await.unwrap();

    assert!(!schema.tables.is_empty());
}
```

#### 9.5.3 MongoDB 容器化集成测试（v1.3 扩展）

```rust
// tests/integration_mongodb.rs — 未来扩展

use testcontainers::runners::AsyncRunner;
use testcontainers_modules::mongodb::MongoDb;

#[tokio::test]
async fn should_read_collections_from_mongodb_container() {
    // testcontainers 自动拉起 MongoDB Docker 容器
    let mongodb = MongoDb::default().start().await.unwrap();
    let conn_str = format!(
        "mongodb://root:root@{}:{}/",
        mongodb.get_host().await.unwrap(),
        mongodb.get_host_port_ipv4(27017).await.unwrap()
    );

    // 使用 MongoDB 适配器读取
    let reader = MongoAdapter::new(&conn_str).await.unwrap();
    let schema = reader.read_schema("test_db").await.unwrap();

    assert!(!schema.tables.is_empty());
}
```

#### 9.5.4 容器化测试基础设施封装

```rust
// tests/common/mod.rs — 测试辅助模块

/// 测试容器辅助函数，统一管理容器生命周期
pub struct TestDatabase {
    pub container: testcontainers::ContainerAsync<Postgres>,
    pub connection_string: String,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let container = Postgres::default()
            .start()
            .await
            .expect("Failed to start PostgreSQL container");

        let conn_str = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap()
        );

        Self { container, connection_string: conn_str }
    }

    /// 执行初始化 SQL
    pub async fn execute_sql(&self, sql: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (client, connection) = tokio_postgres::connect(
            &self.connection_string,
            tokio_postgres::NoTls,
        ).await?;
        tokio::spawn(async move { if let Err(e) = connection.await { eprintln!("DB error: {e}"); } });
        client.batch_execute(sql).await?;
        Ok(())
    }
}

// 每个测试函数自动获得独立的容器，测试之间零干扰
// 容器在测试结束后自动销毁（Drop trait）
```

### 9.6 快照测试（确定性输出）

```rust
#[test]
fn should_produce_stable_markdown_output() {
    // 使用固定的 Schema fixture，确保输出不随代码修改意外变化
    let schema = load_test_fixture("test_schema.json");
    let gen = MarkdownGenerator::new();
    let md = gen.generate(&schema).unwrap();

    // 与预存的期望输出对比
    let expected = include_str!("../fixtures/expected_output.md");
    assert_eq!(md, expected);
}
```

### 9.7 容器化测试的优势

| 对比项 | 传统方式（本地安装数据库） | 容器化方式（testcontainers） |
|--------|--------------------------|---------------------------|
| **环境搭建** | 需手动安装 PostgreSQL/MySQL/MongoDB | 零安装，Docker 自动拉起 |
| **版本管理** | 系统全局一份，版本冲突 | 每个测试指定精确版本 |
| **测试隔离** | 测试间共享数据，可能互相影响 | 每个测试独立容器，零干扰 |
| **CI 一致性** | 需配置 CI 的数据库服务 | 本地和 CI 使用相同容器镜像 |
| **多数据库测试** | 需安装多个数据库 | Docker Compose 一键切换 |
| **数据清理** | 手动清理或事务回滚 | 容器销毁即清理 |

---

## 十、部署方案

### 10.1 MVP 分发方式

#### 10.1.1 静态二进制（推荐）

**目标用户操作**：

```bash
# 下载
wget https://github.com/MasterHesse/crab-shell/releases/download/v0.1.0/crab-shell-linux-x86_64

# 运行
chmod +x crab-shell-linux-x86_64
./crab-shell-linux-x86_64 --version
./crab-shell-linux-x86_64 generate --url postgres://... --output ./docs

# 可选：安装到 PATH
sudo mv crab-shell-linux-x86_64 /usr/local/bin/crab-shell
crab-shell --version
```

**优势**：
- 单文件，零依赖
- < 10 MB 下载
- 瞬间启动

#### 10.1.2 cargo install（Rust 用户）

```bash
cargo install crab-shell --git https://github.com/MasterHesse/crab-shell
```

#### 10.1.3 Docker 镜像（CI/CD 场景）

**Dockerfile（多阶段构建）**：

```dockerfile
# ====== 阶段 1: 编译 ======
FROM rust:1.77-alpine AS builder
RUN apk add --no-cache musl-dev pkg-config openssl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# 利用 Docker 缓存层：先复制依赖文件，编译依赖
RUN cargo build --release 2>/dev/null || true
COPY src ./src
COPY templates ./templates
RUN cargo build --release

# ====== 阶段 2: 运行时（最小镜像） ======
FROM alpine:3.19
RUN apk add --no-cache ca-certificates libpq
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/crab-shell /usr/local/bin/
ENTRYPOINT ["crab-shell"]
```

**Docker 使用方式**：

```bash
# 构建镜像
docker build -t crab-shell:latest .

# 直接运行（连接外部 PostgreSQL）
docker run --rm crab-shell:latest generate \
  --url postgres://user:pass@host:5432/mydb \
  --output /docs
docker run --rm -v $(pwd)/output:/docs crab-shell:latest generate \
  --url postgres://user:pass@host:5432/mydb \
  --output /docs

# 配合 Docker Compose 网络直接访问数据库容器
# docker-compose.yml 中已定义 postgres 服务
docker compose exec crab-shell crab-shell generate \
  --url postgres://hesse:hesse@postgres:5432/crab_shell_test \
  --output /app/output
```

**最终镜像体积**：~20 MB（对比 Java 版 250 MB）

#### 10.1.4 GitHub Release

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build-release:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
            artifact: crab-shell-linux-x86_64
          - target: x86_64-apple-darwin
            os: macos-latest
            artifact: crab-shell-macos-x86_64
          - target: aarch64-apple-darwin
            os: macos-latest
            artifact: crab-shell-macos-aarch64
          - target: x86_64-pc-windows-gnu
            os: ubuntu-latest
            artifact: crab-shell-windows-x86_64.exe
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      - name: Upload
        uses: softprops/action-gh-release@v1
        with:
          files: target/${{ matrix.target }}/release/crab-shell${{ contains(matrix.artifact, 'windows') && '.exe' || '' }}
```

### 10.2 CI/CD Pipeline（容器化）

CI/CD 使用 **Docker-in-Docker** 支持 testcontainers，同时提供 **service containers** 作为备选。

#### 10.2.1 主 CI Pipeline（testcontainers 模式）

```yaml
# .github/workflows/ci.yml
name: CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Docker-in-Docker：让 testcontainers 在 CI 中工作
      - name: Set up Docker
        uses: docker/setup-buildx-action@v3

      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      # 缓存 Docker 镜像层，加速 testcontainers 测试
      - name: Cache Docker images
        uses: actions/cache@v4
        with:
          path: /tmp/.docker-cache
          key: docker-${{ hashFiles('Cargo.toml') }}
          restore-keys: docker-

      - name: Pre-pull testcontainers images
        run: |
          docker pull postgres:16-alpine
          docker pull mysql:8.0
          docker pull mongo:7

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Run tests (testcontainers 自动管理数据库容器)
        run: cargo test --all -- --nocapture

      - name: Build release
        run: cargo build --release

  # 多平台交叉编译检查
  cross-compile:
    needs: test
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-musl
          - aarch64-unknown-linux-gnu
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
```

#### 10.2.2 轻量 CI Pipeline（service containers 模式）

备选方案，不使用 testcontainers，直接用 GitHub Actions service containers：

```yaml
# .github/workflows/ci-lightweight.yml（备选）
name: CI (Lightweight)
on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_USER: hesse
          POSTGRES_PASSWORD: hesse
          POSTGRES_DB: crab_shell_test
        options: >-
          --health-cmd pg_isready
          --health-interval 5s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Run tests
        env:
          DATABASE_URL: postgres://hesse:hesse@localhost:5432/crab_shell_test
        run: cargo test --all

      - name: Build release
        run: cargo build --release
```

#### 10.2.3 CI/CD 环境一致性说明

```
┌─────────────────────────────────────────────────────┐
│              环境一致性保证                           │
│                                                     │
│  本地开发        CI Pipeline      Release Docker     │
│  ─────────      ─────────────    ──────────────     │
│  docker compose  testcontainers   Dockerfile         │
│       │              │               │               │
│       ▼              ▼               ▼               │
│  ┌──────────────────────────────────────┐           │
│  │  PostgreSQL 16-alpine (同一镜像)      │           │
│  │  MySQL 8.0         (同一镜像)        │           │
│  │  MongoDB 7         (同一镜像)        │           │
│  └──────────────────────────────────────┘           │
│                                                     │
│  → 本地通过 = CI 通过 = 部署通过                      │
└─────────────────────────────────────────────────────┘
```

---

## 十一、项目里程碑

### 11.1 里程碑时间表

| 里程碑 | 天数 | 交付物 | 成功标准 |
|--------|------|--------|----------|
| **M1: MVP** | Day 1-8 | CLI 二进制 + README | 能成功生成 Markdown 文档和 ER 图 |
| **M2: 内测反馈** | Day 9-14 | Bug 修复 + 改进 | 3 个真实用户测试通过 |
| **M3: v1.0 发布** | Day 15-20 | HTML 输出 + 模板系统 | 支持可交互 HTML 文档 |
| **M4: v1.1 发布** | Day 21-26 | Schema Diff + CI/CD | 支持变更追踪 |
| **M5: v1.2 发布** | Day 27-30 | PDF 输出 + 品牌定制 | 完整功能集 |
| **M6: 社区推广** | 持续 | 博客 + GitHub README | GitHub 100+ stars |

### 11.2 甘特图

```
Week 1          Week 2          Week 3          Week 4          Week 5
Mon..Fri        Mon..Fri        Mon..Fri        Mon..Fri        Mon..Fri

[MVP - Day 1~8]
████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
 D1 D2 D3 D4 D5 D6 D7 D8

[内测 - Day 9~14]
░░░░░░░░░░░░░░░████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
                D9 D10 D11 D12 D13 D14

[v1.0 - Day 15~20]
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████████████████░░░░░░░░░░░░░░░░░░
                                D15 D16 D17 D18 D19 D20

[v1.1 - Day 21~26]
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████████████████░░░░
                                                D21 D22 D23 D24 D25 D26

[v1.2 - Day 27~30]
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████
                                                                        D27 D28 D29 D30
```

---

## 十二、风险评估与应对

### 12.1 个人开发风险

| 风险 | 概率 | 影响 | 缓解策略 |
|------|------|------|----------|
| **单人开发进度延迟** | 高 | 中 | 合理规划 MVP 范围，采用 Lean Agile 灵活调整优先级 |
| **Rust 学习曲线** | 中 | 中 | 先用简单模块练手（CLI 骨架、Markdown 生成），数据库模块参考 tokio-postgres 官方示例 |
| **缺乏 Code Review** | 中 | 低 | 使用 clippy 严格检查 + 定期自审 + 未来可邀请社区 review |
| **测试覆盖不足** | 中 | 中 | 核心模块先写测试，CI 强制 test 通过 |

### 12.2 技术风险

| 风险 | 概率 | 影响 | 缓解策略 |
|------|------|------|----------|
| **ER 图复杂度爆炸**（>50 表） | 高 | 高 | 分 schema 渲染；提供 `--max-tables` 过滤；支持 DOT 输出 |
| **PostgreSQL 版本兼容** | 中 | 中 | 使用 `information_schema` 标准接口；CI 测试 PG 12-16 |
| **交叉编译问题** | 中 | 低 | 使用 musl target；CI 多目标构建验证 |
| **tokio-postgres 异步复杂度** | 中 | 中 | 参考官方示例；连接池使用 deadpool-postgres |

### 12.3 市场风险

| 风险 | 概率 | 影响 | 缓解策略 |
|------|------|------|----------|
| **获客困难** | 高 | 高 | 开源快速获取用户；在掘金/CSDN/Dev.to 发技术文章 |
| **付费转化率低** | 中 | 高 | Freemium 模式；核心功能免费，高级输出格式付费 |
| **大厂入局** | 低 | 高 | 快速占领细分市场；专注"交付场景"差异化 |

---

## 十三、附录

### 13.1 术语表

| 术语 | 定义 |
|------|------|
| **Schema** | 数据库模式，包含表、视图、索引等对象的集合 |
| **ER 图** | Entity-Relationship Diagram，实体关系图 |
| **数据字典** | 描述数据库中所有表的详细信息的文档 |
| **Snapshot** | 快照，保存某一时刻的 Schema 状态 |
| **Diff** | 对比两个快照，找出差异 |
| **CLI** | Command Line Interface，命令行界面 |
| **TUI** | Text-based User Interface，文本用户界面 |
| **MVP** | Minimum Viable Product，最小可行产品 |

### 13.2 Rust 生态参考

| 优秀 CLI 工具（Rust 编写） | 项目地址 | 学习要点 |
|---------------------------|----------|----------|
| ripgrep (rg) | github.com/BurntSushi/ripgrep | CLI 架构、并行处理、ignore 机制 |
| bat | github.com/sharkdp/bat | 语法高亮集成、输出格式化 |
| fd | github.com/sharkdp/fd | glob 模式匹配、CLI 用户体验 |
| lazygit | github.com/jesseduffield/lazygit | TUI 实战、终端交互 |
| sqlx | github.com/launchbadge/sqlx | 数据库连接、编译时 SQL 检查 |

### 13.3 参考文档

1. [tokio-postgres 文档](https://docs.rs/tokio-postgres)
2. [clap 文档](https://docs.rs/clap)
3. [ratatui 文档](https://docs.rs/ratatui)（TUI 进阶参考）
4. [Tera 模板文档](https://tera.netlify.app/)
5. [PostgreSQL information_schema](https://www.postgresql.org/docs/current/information-schema.html)

### 13.4 未来扩展方向

1. **TUI 界面**：基于 ratatui 的终端浏览界面
2. **多数据库支持**：MySQL（sqlx）、SQLite
3. **AI 注释生成**：集成 LLM 自动生成表/列注释
4. **数据血缘分析**：追踪数据流转路径
5. **性能建议**：基于 Schema 提供索引优化建议
6. **VSCode 插件**：IDE 内直接预览 Schema 文档

---

**文档结束**

_本文档为个人独立开发项目设计，技术栈从 Java 切换为 Rust，开发环境基于 WSL Ubuntu。采用 DDD（领域驱动设计）模块化架构、TDD（测试驱动开发）方法论、Docker Compose 全栈容器化部署。_
