-- devEnvLite 中的新脚本。
-- 
-- 日期：2025年7月15日
-- 
-- 时间：16:04:18
-- 环境变量表
CREATE TABLE environment_variable (
    id TEXT PRIMARY KEY NOT NULL,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    description TEXT
);

-- 环境变量配置表
CREATE TABLE env_config (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    scope TEXT NOT NULL CHECK (scope IN ('system', 'user')), -- 作用域: system或user
    description TEXT,
    sort INTEGER DEFAULT 1
);

-- 分组表
CREATE TABLE variable_group (
    id TEXT PRIMARY KEY NOT NULL,
    config_id TEXT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    sort INTEGER DEFAULT 1,
    FOREIGN KEY (config_id) REFERENCES env_config(id)
);

-- 环境变量-分组关联表
CREATE TABLE variable_group_mapping (
	group_id TEXT NOT NULL,
    variable_id TEXT NOT NULL,
    sort INTEGER DEFAULT 1,
    PRIMARY KEY (variable_id, group_id),
    FOREIGN KEY (variable_id) REFERENCES environment_variable(id),
    FOREIGN KEY (group_id) REFERENCES variable_group(id)
);
