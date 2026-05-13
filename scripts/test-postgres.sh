#!/bin/bash
# 脚本：使用 Docker exec 验证 PostgreSQL Schema 读取功能

set -e

echo "=== PostgreSQL Schema 验证测试 ==="
echo

# 检查数据库连接
echo "1. 检查数据库连接..."
docker compose exec -T postgres psql -U hesse -d crab_shell_test -c "SELECT 1;" > /dev/null 2>&1
echo "✓ 数据库连接正常"
echo

# 检查表数量
echo "2. 检查表数量..."
COUNT=$(docker compose exec -T postgres psql -U hesse -d crab_shell_test -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" | tr -d ' ')
if [ "$COUNT" = "5" ]; then
    echo "✓ 表数量正确: $COUNT"
else
    echo "✗ 表数量错误: 期望 5, 实际 $COUNT"
    exit 1
fi
echo

# 检查表注释
echo "3. 检查表注释..."
COMMENT=$(docker compose exec -T postgres psql -U hesse -d crab_shell_test -t -c "SELECT obj_description((table_schema || '.' || table_name)::regclass, 'pg_class') FROM information_schema.tables WHERE table_name = 'users' AND table_schema = 'public';" | tr -d ' ')
if [ "$COMMENT" = "用户信息表" ]; then
    echo "✓ users 表注释正确: $COMMENT"
else
    echo "✗ users 表注释错误: 期望 '用户信息表', 实际 '$COMMENT'"
    exit 1
fi
echo

# 检查列数量
echo "4. 检查列数量..."
COUNT=$(docker compose exec -T postgres psql -U hesse -d crab_shell_test -t -c "SELECT COUNT(*) FROM information_schema.columns WHERE table_schema = 'public' AND table_name = 'users';" | tr -d ' ')
if [ "$COUNT" = "4" ]; then
    echo "✓ users 表列数正确: $COUNT"
else
    echo "✗ users 表列数错误: 期望 4, 实际 $COUNT"
    exit 1
fi
echo

# 检查主键
echo "5. 检查主键..."
PK=$(docker compose exec -T postgres psql -U hesse -d crab_shell_test -t -c "SELECT kcu.column_name FROM information_schema.table_constraints tc JOIN information_schema.key_column_usage kcu ON tc.constraint_name = kcu.constraint_name WHERE tc.table_schema = 'public' AND tc.table_name = 'users' AND tc.constraint_type = 'PRIMARY KEY';" | tr -d ' ')
if [ "$PK" = "id" ]; then
    echo "✓ users 表主键正确: $PK"
else
    echo "✗ users 表主键错误: 期望 'id', 实际 '$PK'"
    exit 1
fi
echo

# 检查外键
echo "6. 检查外键..."
FK_COUNT=$(docker compose exec -T postgres psql -U hesse -d crab_shell_test -t -c "SELECT COUNT(*) FROM information_schema.table_constraints WHERE table_schema = 'public' AND table_name = 'orders' AND constraint_type = 'FOREIGN KEY';" | tr -d ' ')
if [ "$FK_COUNT" = "1" ]; then
    echo "✓ orders 表外键数量正确: $FK_COUNT"
else
    echo "✗ orders 表外键数量错误: 期望 1, 实际 $FK_COUNT"
    exit 1
fi
echo

# 检查 CHECK 约束
echo "7. 检查 CHECK 约束..."
CHECK_COUNT=$(docker compose exec -T postgres psql -U hesse -d crab_shell_test -t -c "SELECT COUNT(*) FROM pg_constraint WHERE conrelid = (SELECT oid FROM pg_class WHERE relname = 'orders') AND contype = 'c';" | tr -d ' ')
if [ "$CHECK_COUNT" = "1" ]; then
    echo "✓ orders 表 CHECK 约束数量正确: $CHECK_COUNT"
else
    echo "✗ orders 表 CHECK 约束数量错误: 期望 1, 实际 $CHECK_COUNT"
    exit 1
fi
echo

# 检查索引
echo "8. 检查索引..."
IDX_COUNT=$(docker compose exec -T postgres psql -U hesse -d crab_shell_test -t -c "SELECT COUNT(*) FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'orders';" | tr -d ' ')
if [ "$IDX_COUNT" = "3" ]; then
    echo "✓ orders 表索引数量正确: $IDX_COUNT"
else
    echo "✗ orders 表索引数量错误: 期望 3, 实际 $IDX_COUNT"
    exit 1
fi
echo

echo "=== 所有测试通过 ==="
