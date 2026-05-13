-- tests/test_setup.sql
-- Crab Shell 测试数据库初始化脚本
-- 用于创建测试表并插入测试数据

-- ============================================
-- 1. 创建测试 schema（如果不存在）
-- ============================================
CREATE SCHEMA IF NOT EXISTS crab_test;

-- 设置 search_path
SET search_path TO crab_test, public;

-- ============================================
-- 2. 创建枚举类型
-- ============================================
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('admin', 'editor', 'viewer');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE order_status AS ENUM ('pending', 'paid', 'shipped', 'cancelled');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- 3. 创建测试表 - users 表
-- ============================================
DROP TABLE IF EXISTS crab_test.order_items CASCADE;
DROP TABLE IF EXISTS crab_test.orders CASCADE;
DROP TABLE IF EXISTS crab_test.products CASCADE;
DROP TABLE IF EXISTS crab_test.categories CASCADE;
DROP TABLE IF EXISTS crab_test.users CASCADE;

-- 用户表
CREATE TABLE crab_test.users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'viewer',
    is_active BOOLEAN NOT NULL DEFAULT true,
    login_count INT NOT NULL DEFAULT 0,
    last_login TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 添加表注释
COMMENT ON TABLE crab_test.users IS '用户账户信息表';
COMMENT ON COLUMN crab_test.users.id IS '用户唯一标识';
COMMENT ON COLUMN crab_test.users.username IS '用户名（唯一）';
COMMENT ON COLUMN crab_test.users.email IS '用户邮箱（唯一）';
COMMENT ON COLUMN crab_test.users.role IS '用户角色';
COMMENT ON COLUMN crab_test.users.is_active IS '账户是否激活';

-- 创建更新时间触发器函数
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器
CREATE TRIGGER update_users_updated_at BEFORE UPDATE
    ON crab_test.users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- 4. 创建测试表 - categories 表
-- ============================================
CREATE TABLE crab_test.categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    parent_id INT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE crab_test.categories IS '商品分类表';
COMMENT ON COLUMN crab_test.categories.parent_id IS '父分类ID（自引用）';

-- 自引用外键
ALTER TABLE crab_test.categories ADD CONSTRAINT fk_category_parent
    FOREIGN KEY (parent_id) REFERENCES crab_test.categories(id);

-- ============================================
-- 5. 创建测试表 - products 表
-- ============================================
CREATE TABLE crab_test.products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    category_id INT NOT NULL,
    price DECIMAL(10, 2) NOT NULL CHECK (price >= 0),
    stock_quantity INT NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    sku VARCHAR(50) NOT NULL UNIQUE,
    is_available BOOLEAN NOT NULL DEFAULT true,
    tags TEXT[], -- PostgreSQL 数组类型
    metadata JSONB, -- JSONB 类型
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE crab_test.products IS '商品信息表';

-- 外键：商品 -> 分类
ALTER TABLE crab_test.products ADD CONSTRAINT fk_product_category
    FOREIGN KEY (category_id) REFERENCES crab_test.categories(id);

-- 索引
CREATE INDEX idx_products_category ON crab_test.products(category_id);
CREATE INDEX idx_products_sku ON crab_test.products(sku);
CREATE INDEX idx_products_available ON crab_test.products(is_available);

-- 触发器
CREATE TRIGGER update_products_updated_at BEFORE UPDATE
    ON crab_test.products FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- 6. 创建测试表 - orders 表
-- ============================================
CREATE TABLE crab_test.orders (
    id SERIAL PRIMARY KEY,
    order_no VARCHAR(32) NOT NULL UNIQUE,
    user_id INT NOT NULL,
    status order_status NOT NULL DEFAULT 'pending',
    total_amount DECIMAL(12, 2) NOT NULL DEFAULT 0.00,
    shipping_address TEXT NOT NULL,
    remark TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE crab_test.orders IS '订单主表';

-- 外键：订单 -> 用户
ALTER TABLE crab_test.orders ADD CONSTRAINT fk_order_user
    FOREIGN KEY (user_id) REFERENCES crab_test.users(id);

-- 索引
CREATE INDEX idx_orders_user ON crab_test.orders(user_id);
CREATE INDEX idx_orders_status ON crab_test.orders(status);
CREATE INDEX idx_orders_created ON crab_test.orders(created_at DESC);

-- 触发器
CREATE TRIGGER update_orders_updated_at BEFORE UPDATE
    ON crab_test.orders FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- 7. 创建测试表 - order_items 表
-- ============================================
CREATE TABLE crab_test.order_items (
    id SERIAL PRIMARY KEY,
    order_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL CHECK (quantity > 0),
    unit_price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE crab_test.order_items IS '订单明细表';

-- 外键：订单明细 -> 订单
ALTER TABLE crab_test.order_items ADD CONSTRAINT fk_order_item_order
    FOREIGN KEY (order_id) REFERENCES crab_test.orders(id) ON DELETE CASCADE;

-- 外键：订单明细 -> 商品
ALTER TABLE crab_test.order_items ADD CONSTRAINT fk_order_item_product
    FOREIGN KEY (product_id) REFERENCES crab_test.products(id);

-- 复合唯一约束
ALTER TABLE crab_test.order_items ADD CONSTRAINT uk_order_product
    UNIQUE (order_id, product_id);

-- 索引
CREATE INDEX idx_order_items_order ON crab_test.order_items(order_id);
CREATE INDEX idx_order_items_product ON crab_test.order_items(product_id);

-- ============================================
-- 8. 创建视图
-- ============================================
CREATE OR REPLACE VIEW crab_test.v_user_order_summary AS
SELECT
    u.id AS user_id,
    u.username,
    COUNT(o.id) AS order_count,
    COALESCE(SUM(o.total_amount), 0) AS total_spent
FROM crab_test.users u
LEFT JOIN crab_test.orders o ON u.id = o.user_id
GROUP BY u.id, u.username;

COMMENT ON VIEW crab_test.v_user_order_summary IS '用户订单汇总视图';

-- ============================================
-- 9. 插入测试数据 - users
-- ============================================
INSERT INTO crab_test.users (username, email, password_hash, role, is_active, login_count, last_login) VALUES
('admin', 'admin@example.com', '$2b$12$hashed_password_admin', 'admin', true, 150, '2026-05-13 10:00:00'),
('alice', 'alice@example.com', '$2b$12$hashed_password_alice', 'editor', true, 42, '2026-05-12 15:30:00'),
('bob', 'bob@example.com', '$2b$12$hashed_password_bob', 'viewer', true, 8, '2026-05-10 09:15:00'),
('charlie', 'charlie@example.com', '$2b$12$hashed_password_charlie', 'editor', false, 0, NULL),
('diana', 'diana@example.com', '$2b$12$hashed_password_diana', 'viewer', true, 23, '2026-05-11 11:45:00');

-- ============================================
-- 10. 插入测试数据 - categories
-- ============================================
INSERT INTO crab_test.categories (name, description, parent_id) VALUES
('电子产品', '各类电子产品', NULL),
('服装鞋帽', '服装、鞋类、帽子等', NULL),
('家居生活', '家居生活用品', NULL),
('手机', '智能手机', 1),
('笔记本电脑', '笔记本电脑', 1),
('男装', '男士服装', 2),
('女装', '女士服装', 2);

-- ============================================
-- 11. 插入测试数据 - products
-- ============================================
INSERT INTO crab_test.products (name, description, category_id, price, stock_quantity, sku, is_available, tags, metadata) VALUES
('iPhone 16 Pro', '苹果最新旗舰手机', 4, 8999.00, 50, 'IPHONE-16-PRO', true, ARRAY['手机', '苹果', '旗舰'], '{"brand": "Apple", "warranty": "1年"}'),
('MacBook Pro M4', '搭载 M4 芯片的 MacBook Pro', 5, 18999.00, 30, 'MACBOOK-PRO-M4', true, ARRAY['笔记本', '苹果', 'M4'], '{"brand": "Apple", "cpu": "M4"}'),
('ThinkPad X1', '联想 ThinkPad X1 Carbon', 5, 12999.00, 20, 'THINKPAD-X1', true, ARRAY['笔记本', '联想', '商务'], '{"brand": "Lenovo", "cpu": "Intel i7"}'),
('男士T恤', '纯棉男士短袖T恤', 6, 99.00, 200, 'TSHIRT-M-001', true, ARRAY['服装', 'T恤'], '{"material": "棉"}'),
('女士连衣裙', '夏季新款连衣裙', 7, 299.00, 100, 'DRESS-W-001', true, ARRAY['服装', '连衣裙'], '{"material": "聚酯纤维"}'),
('智能手表', '运动健康监测手表', 4, 1299.00, 80, 'WATCH-001', true, ARRAY['手表', '智能'], '{"brand": "Generic", "warranty": "1年"}');

-- ============================================
-- 12. 插入测试数据 - orders
-- ============================================
INSERT INTO crab_test.orders (order_no, user_id, status, total_amount, shipping_address, remark) VALUES
('ORD-2026-001', 2, 'paid', 9098.00, '北京市海淀区中关村大街1号', '尽快发货'),
('ORD-2026-002', 2, 'shipped', 12999.00, '北京市海淀区中关村大街1号', NULL),
('ORD-2026-003', 5, 'pending', 299.00, '上海市浦东新区陆家嘴环路1号', '周末送货'),
('ORD-2026-004', 2, 'paid', 1299.00, '北京市海淀区中关村大街1号', NULL);

-- ============================================
-- 13. 插入测试数据 - order_items
-- ============================================
INSERT INTO crab_test.order_items (order_id, product_id, quantity, unit_price) VALUES
(1, 1, 1, 8999.00),
(1, 6, 1, 1299.00),
(2, 3, 1, 12999.00),
(3, 5, 1, 299.00),
(4, 6, 1, 1299.00);

-- ============================================
-- 完成提示
-- ============================================
DO $$ BEGIN
    RAISE NOTICE 'Crab Shell 测试数据初始化完成！';
    RAISE NOTICE 'Schema: crab_test';
    RAISE NOTICE 'Tables: users, categories, products, orders, order_items';
    RAISE NOTICE 'View: v_user_order_summary';
END $$;
