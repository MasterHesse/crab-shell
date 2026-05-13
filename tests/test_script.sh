#!/bin/bash
# tests/test_script.sh
# Crab Shell - PostgreSQL 测试脚本
# 用于测试 Docker 中运行的 PostgreSQL 容器

set -e  # 遇到错误立即退出

# ============================================
# 配置变量
# ============================================
CONTAINER_NAME="crab-shell-postgres"
DB_NAME="crab_shell_test"
DB_USER="postgres"
DB_PASSWORD="postgres"
TEST_SCHEMA="crab_test"

# ============================================
# 颜色输出
# ============================================
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# ============================================
# 1. 检查容器状态
# ============================================
log_info "检查 Docker 容器状态..."
if ! docker ps | grep -q "$CONTAINER_NAME"; then
    log_error "容器 $CONTAINER_NAME 未运行！"
    log_info "请先启动容器："
    echo "  docker-compose up -d postgres"
    exit 1
fi
log_info "容器 $CONTAINER_NAME 正在运行 ✓"

# ============================================
# 2. 显示连接命令
# ============================================
log_info "============================================"
log_info "PostgreSQL 连接命令"
log_info "============================================"
echo ""
log_info "方式 1: 使用 docker exec 进入容器并连接 psql"
echo "  docker exec -it $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME"
echo ""
log_info "方式 2: 使用 psql 客户端连接（需要本地安装）"
echo "  psql -h localhost -p 5432 -U $DB_USER -d $DB_NAME"
echo ""
log_info "方式 3: 使用 docker exec 执行 SQL 文件"
echo "  cat tests/test_setup.sql | docker exec -i $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME"
echo ""

# ============================================
# 3. 初始化测试数据库
# ============================================
log_info "============================================"
log_info "初始化测试数据库"
log_info "============================================"

# 检查数据库是否存在，不存在则创建
log_info "检查数据库 $DB_NAME 是否存在..."
if ! docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
    log_warn "数据库 $DB_NAME 不存在，正在创建..."
    docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -c "CREATE DATABASE $DB_NAME;"
    log_info "数据库 $DB_NAME 创建成功 ✓"
else
    log_info "数据库 $DB_NAME 已存在 ✓"
fi

# 执行测试 SQL 脚本
log_info "执行测试数据初始化脚本..."
docker exec -i "$CONTAINER_NAME" psql -U "$DB_USER" -d "$DB_NAME" < "$(dirname "$0")/test_setup.sql"

log_info "测试数据初始化完成 ✓"

# ============================================
# 4. 验证测试数据
# ============================================
log_info "============================================"
log_info "验证测试数据"
log_info "============================================"

log_info "查询 users 表："
docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -d "$DB_NAME" -c "SELECT id, username, email, role, is_active FROM crab_test.users;"

log_info "查询 products 表："
docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -d "$DB_NAME" -c "SELECT id, name, price, stock_quantity FROM crab_test.products;"

log_info "查询视图 v_user_order_summary："
docker exec "$CONTAINER_NAME" psql -U "$DB_USER" -d "$DB_NAME" -c "SELECT * FROM crab_test.v_user_order_summary;"

# ============================================
# 5. 显示项目测试指令
# ============================================
log_info "============================================"
log_info "Crab Shell 项目测试指令"
log_info "============================================"
echo ""
log_info "方式 1: 生成 Markdown 文档"
echo "  export DATABASE_URL=\"postgres://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME\""
echo "  cargo run -- generate --url \"\$DATABASE_URL\" --schema \"$TEST_SCHEMA\" --format markdown --output ./output"
echo ""
log_info "方式 2: 生成 Mermaid ER 图"
echo "  cargo run -- generate --url \"\$DATABASE_URL\" --schema \"$TEST_SCHEMA\" --format mermaid --output ./output"
echo ""
log_info "方式 3: 生成 HTML 文档"
echo "  cargo run -- generate --url \"\$DATABASE_URL\" --schema \"$TEST_SCHEMA\" --format html --output ./output"
echo ""
log_info "方式 4: 创建快照"
echo "  cargo run -- snapshot take --url \"\$DATABASE_URL\" --schema \"$TEST_SCHEMA\" --output ./snapshots/snapshot1.json"
echo ""
log_info "方式 5: 对比两个快照"
echo "  cargo run -- diff ./snapshots/snapshot1.json ./snapshots/snapshot2.json --output ./output/diff.md"
echo ""
log_info "方式 6: 查看 CLI 帮助"
echo "  cargo run -- --help"
echo "  cargo run -- generate --help"
echo ""

# ============================================
# 6. 一键测试命令（可直接复制执行）
# ============================================
log_info "============================================"
log_info "一键测试命令（复制执行）"
log_info "============================================"
cat << 'EOF_TEST'
# 设置数据库连接 URL
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/crab_shell_test"

# 创建输出目录
mkdir -p output snapshots

# 生成 Markdown 文档
cargo run -- generate \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --format markdown \
  --output ./output

# 生成 Mermaid ER 图
cargo run -- generate \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --format mermaid \
  --output ./output

# 生成 HTML 文档
cargo run -- generate \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --format html \
  --output ./output

# 创建快照
cargo run -- snapshot take \
  --url "$DATABASE_URL" \
  --schema "crab_test" \
  --output ./snapshots/snapshot1.json

echo "测试完成！请查看 output/ 和 snapshots/ 目录"
EOF_TEST

log_info "============================================"
log_info "测试脚本执行完成！"
log_info "============================================"
