# Crab Shell - 测试指南

## 测试环境

- **Docker 容器名**: `crab-shell-postgres`
- **镜像**: `postgres:16-alpine`
- **端口**: `5432`
- **用户名**: `postgres`
- **密码**: `postgres`
- **测试数据库**: `crab_shell_test`
- **测试 Schema**: `crab_test`

---

## 1. Docker 容器连接命令

### 方式 1: 进入容器并连接 psql（交互式）

```bash
# 进入容器
docker exec -it crab-shell-postgres bash

# 在容器内连接 psql
psql -U postgres -d crab_shell_test
```

### 方式 2: 直接连接 psql（一步到位）

```bash
docker exec -it crab-shell-postgres psql -U postgres -d crab_shell_test
```

### 方式 3: 使用本地 psql 客户端连接

```bash
psql -h localhost -p 5432 -U postgres -d crab_shell_test
# 密码：postgres
```

### 方式 4: 执行单条 SQL 命令

```bash
docker exec crab-shell-postgres psql -U postgres -d crab_shell_test -c "\dt crab_test.*"
```

---

## 2. 初始化测试数据

### 一键初始化

```bash
# 执行测试脚本（自动创建数据库和测试数据）
./tests/test_script.sh
```

### 手动初始化

```bash
# 1. 创建数据库（如果不存在）
docker exec -it crab-shell-postgres psql -U postgres -c "CREATE DATABASE crab_shell_test;"

# 2. 执行测试数据脚本
cat tests/test_setup.sql | docker exec -i crab-shell-postgres psql -U postgres -d crab_shell_test
```

---

## 3. 测试表结构说明

### users（用户表）
| 列名 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | SERIAL | PRIMARY KEY | 用户 ID |
| username | VARCHAR(50) | UNIQUE, NOT NULL | 用户名 |
| email | VARCHAR(100) | UNIQUE, NOT NULL | 邮箱 |
| password_hash | VARCHAR(255) | NOT NULL | 密码哈希 |
| role | ENUM | NOT NULL, DEFAULT 'viewer' | 角色（admin/editor/viewer） |
| is_active | BOOLEAN | NOT NULL, DEFAULT true | 是否激活 |
| login_count | INT | NOT NULL, DEFAULT 0 | 登录次数 |
| last_login | TIMESTAMP | - | 最后登录时间 |
| created_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 创建时间 |
| updated_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 更新时间（自动更新） |

### categories（分类表）
| 列名 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | SERIAL | PRIMARY KEY | 分类 ID |
| name | VARCHAR(100) | UNIQUE, NOT NULL | 分类名称 |
| description | TEXT | - | 描述 |
| parent_id | INT | FK -> categories(id) | 父分类（自引用） |
| is_active | BOOLEAN | NOT NULL, DEFAULT true | 是否激活 |
| created_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 创建时间 |

### products（商品表）
| 列名 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | SERIAL | PRIMARY KEY | 商品 ID |
| name | VARCHAR(200) | NOT NULL | 商品名称 |
| description | TEXT | - | 描述 |
| category_id | INT | FK -> categories(id) | 分类 ID |
| price | DECIMAL(10,2) | NOT NULL, CHECK >= 0 | 价格 |
| stock_quantity | INT | NOT NULL, DEFAULT 0, CHECK >= 0 | 库存 |
| sku | VARCHAR(50) | UNIQUE, NOT NULL | SKU 编码 |
| is_available | BOOLEAN | NOT NULL, DEFAULT true | 是否上架 |
| tags | TEXT[] | - | 标签（数组类型） |
| metadata | JSONB | - | 元数据（JSONB 类型） |
| created_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 创建时间 |
| updated_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 更新时间（自动更新） |

### orders（订单表）
| 列名 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | SERIAL | PRIMARY KEY | 订单 ID |
| order_no | VARCHAR(32) | UNIQUE, NOT NULL | 订单编号 |
| user_id | INT | FK -> users(id) | 用户 ID |
| status | ENUM | NOT NULL, DEFAULT 'pending' | 状态 |
| total_amount | DECIMAL(12,2) | NOT NULL, DEFAULT 0.00 | 总金额 |
| shipping_address | TEXT | NOT NULL | 收货地址 |
| remark | TEXT | - | 备注 |
| created_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 创建时间 |
| updated_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 更新时间（自动更新） |

### order_items（订单明细表）
| 列名 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | SERIAL | PRIMARY KEY | 明细 ID |
| order_id | INT | FK -> orders(id) ON DELETE CASCADE | 订单 ID |
| product_id | INT | FK -> products(id) | 商品 ID |
| quantity | INT | NOT NULL, CHECK > 0 | 数量 |
| unit_price | DECIMAL(10,2) | NOT NULL | 单价 |
| created_at | TIMESTAMP | NOT NULL, DEFAULT NOW() | 创建时间 |

### 视图: v_user_order_summary
| 列名 | 类型 | 说明 |
|------|------|------|
| user_id | INT | 用户 ID |
| username | VARCHAR(50) | 用户名 |
| order_count | BIGINT | 订单数量 |
| total_spent | NUMERIC | 总消费金额 |

---

## 4. Crab Shell 测试指令

### 设置数据库连接 URL

```bash
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/crab_shell_test"
```

### 生成 Markdown 文档

```bash
cargo run -- generate \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --format markdown \
  --output ./output
```

### 生成 Mermaid ER 图

```bash
cargo run -- generate \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --format mermaid \
  --output ./output
```

### 生成 HTML 文档

```bash
cargo run -- generate \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --format html \
  --output ./output
```

### 创建快照

```bash
cargo run -- snapshot take \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --output ./snapshots/snapshot1.json
```

### 对比两个快照

```bash
# 先修改数据库（添加列、修改类型等）
# 然后创建第二个快照
cargo run -- snapshot take \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --output ./snapshots/snapshot2.json

# 对比两个快照
cargo run -- diff \
  ./snapshots/snapshot1.json \
  ./snapshots/snapshot2.json \
  --output ./output/diff.md
```

### 查看 CLI 帮助

```bash
cargo run -- --help
cargo run -- generate --help
cargo run -- snapshot --help
cargo run -- diff --help
```

---

## 5. 常用 psql 命令

```sql
-- 列出所有表
\dt crab_test.*

-- 查看表结构
\d crab_test.users

-- 查看索引
\di crab_test.*

-- 查看外键约束
\d crab_test.orders

-- 查询数据
SELECT * FROM crab_test.users;
SELECT * FROM crab_test.products;
SELECT * FROM crab_test.v_user_order_summary;

-- 查看视图定义
\d+ crab_test.v_user_order_summary

-- 退出 psql
\q
```

---

## 6. 测试流程图

```
┌─────────────────┐
│  启动 Docker 容器  │
│ docker-compose up │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  初始化测试数据   │
│ ./tests/test_   │
│ script.sh       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  运行 Crab Shell  │
│ cargo run --     │
│  generate ...    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  查看输出文档    │
│ output/         │
└─────────────────┘
```
