-- devEnvLite 中的新脚本。
-- 
-- 日期：2025年7月15日
-- 
-- 时间：16:04:18
-- 环境变量配置系统核心表结构

-- 环境变量基础信息表
-- 存储所有环境变量的键值对及描述信息
CREATE TABLE environment_variable (
    id TEXT PRIMARY KEY NOT NULL,            -- 变量唯一标识，UUID格式
    key TEXT NOT NULL UNIQUE,               -- 变量名，全局唯一
    value TEXT NOT NULL,                    -- 变量值，可为任意文本
    description TEXT                        -- 变量描述信息，用于说明用途
);

-- 环境变量配置表
-- 管理不同作用域的环境变量集合
CREATE TABLE env_config (
    id TEXT PRIMARY KEY NOT NULL,            -- 配置唯一标识，UUID格式
    name TEXT NOT NULL UNIQUE,              -- 配置名称，全局唯一
    scope TEXT NOT NULL CHECK (scope IN ('system', 'user')), -- 作用域: system系统级，user用户级
    description TEXT,                       -- 配置描述信息
    is_active BOOLEAN NOT NULL DEFAULT 1,   -- 是否启用，1-启用，0-禁用
    sort INTEGER DEFAULT 1                  -- 排序权重，数值越小越靠前
);

-- 变量分组表
-- 对环境变量进行逻辑分组，便于管理
CREATE TABLE variable_group (
    id TEXT PRIMARY KEY NOT NULL,            -- 分组唯一标识，UUID格式
    config_id TEXT NOT NULL,                -- 所属配置ID，关联env_config表
    name TEXT NOT NULL UNIQUE,              -- 分组名称，同一配置内唯一
    description TEXT,                       -- 分组描述信息
    sort INTEGER DEFAULT 1,                 -- 排序权重，数值越小越靠前
    FOREIGN KEY (config_id) REFERENCES env_config(id) -- 外键约束，确保引用有效性
);

-- 环境变量-分组关联表
-- 实现变量与分组的多对多关系
CREATE TABLE variable_group_mapping (
    group_id TEXT NOT NULL,                 -- 分组ID
    variable_id TEXT NOT NULL,              -- 变量ID
    sort INTEGER DEFAULT 1,                 -- 排序权重，同一分组内排序
    PRIMARY KEY (variable_id, group_id),    -- 复合主键，确保唯一性
    FOREIGN KEY (variable_id) REFERENCES environment_variable(id), -- 关联变量表
    FOREIGN KEY (group_id) REFERENCES variable_group(id) -- 关联分组表
);
