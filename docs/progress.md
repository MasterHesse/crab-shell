# crab-shell 项目进度

## 概览

**当前阶段**: M1 MVP 核心功能开发  
**进度**: 🔄 进行中  
**测试覆盖**: ✅ 92+ 测试通过

---

## 阶段进度

### M0 项目初始化 ✅ 100%

- [x] 项目骨架搭建
- [x] 领域模型定义
- [x] 基础设施规划

### M1 MVP 核心功能 🔄 进行中

| 任务 | 子任务 | 状态 | 说明 |
|------|--------|------|------|
| T1-1 | SchemaReader trait 和 PostgreSQL 适配器 | ✅ | trait 接口已定义 |
| T1-2 | Markdown 生成器 | ✅ | 34 测试通过 |
| T1-3 | Mermaid ER 图生成器 | ✅ | 17 测试通过 |
| T1-4 | CLI generate 命令 | ✅ | 完整实现 |
| T1-5 | 端到端测试 | ✅ | 12 测试通过 |
| T1-6 | PostgreSQL 集成测试 | 🔄 | 约束读取已实现 |

---

## 测试覆盖

### 单元测试 ✅

| 模块 | 测试数 | 状态 |
|------|--------|------|
| MarkdownGenerator | 34 | ✅ 全部通过 |
| MermaidGenerator | 17 | ✅ 全部通过 |
| SnapshotManager | 12 | ✅ 全部通过 |
| Schema 模型 | 26 | ✅ 全部通过 |
| **总计** | **89** | **✅ 全部通过** |

### 集成测试 🔄

| 测试 | 状态 | 说明 |
|------|------|------|
| PostgreSQL Schema 读取 | ✅ | 使用 `scripts/test-postgres.sh` 验证 |
| Docker 容器 | ✅ | `docker compose up -d postgres` |
| 端到端命令 | 🔄 | CLI 测试待完善 |

---

## 已完成功能

### 核心模块

1. **Schema 领域模型**
   - DatabaseSchema, Table, Column, PrimaryKey, ForeignKey, Index, Constraint
   - View, EnumType 支持

2. **SchemaReader trait**
   - `read_schema(schema_name)` - 读取完整 Schema
   - `read_tables(schema_name)` - 读取表列表

3. **DocumentGenerator trait**
   - `generate(schema)` - 生成文档

4. **PostgreSQL 适配器**
   - 表/列/注释读取
   - 主键、外键、索引读取
   - CHECK/UNIQUE/REFERENCES 约束读取

5. **Markdown 生成器**
   - 表结构文档
   - 列信息
   - 约束信息
   - Mermaid ER 图嵌入

6. **Mermaid 生成器**
   - ER 图
   - 关系箭头

7. **CLI 命令**
   - `generate` - 生成文档
   - `snapshot` - 快照管理
   - `diff` - Schema 差异

8. **Snapshot 管理**
   - JSON 序列化/反序列化
   - Schema 比较

---

## 测试执行

### 运行所有测试
```bash
cargo test
```

### 运行特定测试
```bash
cargo test --test markdown_generator_test
cargo test --test mermaid_generator_test
cargo test --test snapshot_manager_test
cargo test --test schema_model_test
```

### Docker PostgreSQL 验证测试
```bash
# 启动 PostgreSQL
docker compose up -d postgres

# 运行验证脚本
./scripts/test-postgres.sh
```

---

## 待办事项

- [ ] 完善 CLI 端到端测试
- [ ] 添加 MySQL 适配器
- [ ] 添加配置文件支持
- [ ] 添加 CI/CD 配置
- [ ] 完善错误处理

---

## 已知问题与改进建议

### 数据库连接相关

1. **端口显示问题修复**
   - 在 `src/infra/postgres.rs` 和 `src/infra/connection.rs` 中添加 `parse_connection_info()` 函数
   - 解析实际连接信息而非硬编码端口 5432
   - 影响文件：`src/cli/commands.rs`

2. **Docker 端口映射优化**
   - 端口映射改为 `"5433:5432"`（主机 5433 → 容器 5432）
   - 确保 Docker 网络配置正确

3. **PostgreSQL 认证配置**
   - `pg_hba.conf` 添加正确的认证规则
   - 确保 Docker 环境下的客户端连接认证正常

---

## 文档

- [工程文档](./crab-shell-engineering-doc.md)
- [README](../README.md)
- [测试脚本](../scripts/test-postgres.sh)
