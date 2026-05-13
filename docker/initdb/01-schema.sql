-- docker/initdb/01-schema.sql
-- crab-shell 测试数据库初始化脚本

-- 创建测试用户表
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE users IS '用户信息表';
COMMENT ON COLUMN users.id IS '用户ID';
COMMENT ON COLUMN users.username IS '用户名';
COMMENT ON COLUMN users.email IS '邮箱地址';
COMMENT ON COLUMN users.created_at IS '创建时间';

-- 创建订单表
CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    total_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE orders IS '订单表';
COMMENT ON COLUMN orders.id IS '订单ID';
COMMENT ON COLUMN orders.user_id IS '用户ID (外键)';
COMMENT ON COLUMN orders.total_amount IS '订单总金额';
COMMENT ON COLUMN orders.status IS '订单状态';
COMMENT ON COLUMN orders.created_at IS '创建时间';

-- 创建订单项表
CREATE TABLE IF NOT EXISTS order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id),
    product_name VARCHAR(200) NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    price DECIMAL(10,2) NOT NULL
);

COMMENT ON TABLE order_items IS '订单项表';
COMMENT ON COLUMN order_items.id IS '订单项ID';
COMMENT ON COLUMN order_items.order_id IS '订单ID (外键)';
COMMENT ON COLUMN order_items.product_name IS '商品名称';
COMMENT ON COLUMN order_items.quantity IS '数量';
COMMENT ON COLUMN order_items.price IS '单价';

-- 创建产品表
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    category VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE products IS '产品表';
COMMENT ON COLUMN products.id IS '产品ID';
COMMENT ON COLUMN products.name IS '产品名称';
COMMENT ON COLUMN products.description IS '产品描述';
COMMENT ON COLUMN products.price IS '产品价格';
COMMENT ON COLUMN products.category IS '产品分类';
COMMENT ON COLUMN products.created_at IS '创建时间';

-- 创建分类表
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
);

COMMENT ON TABLE categories IS '分类表';
COMMENT ON COLUMN categories.id IS '分类ID';
COMMENT ON COLUMN categories.name IS '分类名称';
COMMENT ON COLUMN categories.description IS '分类描述';

-- 添加索引
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);

-- 添加唯一约束
ALTER TABLE products ADD CONSTRAINT uk_products_name UNIQUE (name);

-- 添加检查约束
ALTER TABLE orders ADD CONSTRAINT chk_orders_status CHECK (status IN ('PENDING', 'PAID', 'SHIPPED', 'DELIVERED', 'CANCELLED'));
ALTER TABLE order_items ADD CONSTRAINT chk_order_items_quantity CHECK (quantity > 0);
ALTER TABLE order_items ADD CONSTRAINT chk_order_items_price CHECK (price >= 0);
ALTER TABLE products ADD CONSTRAINT chk_products_price CHECK (price >= 0);
