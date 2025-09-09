/** @type {import('cz-git').UserConfig} */
module.exports = {
	rules: {
		// @see: https://commitlint.js.org/#/reference-rules
		"body-leading-blank": [2, "always"], // body换行
		"header-max-length": [2, "never", 72], // header 最长72
		// type类型定义，表示git提交的 type 必须在以下类型范围内
		"type-enum": [
			2,
			"always",
			[
				"feat",
				"fix",
				"docs",
				"style",
				"refactor",
				"perf",
				"test",
				"build",
				"ci",
				"revert",
				"chore",
			],
		],
		// 大小写不做校验
		"subject-case": [0],
	},
	prompt: {
		alias: { fd: "docs: fix typos" },
		messages: {
			type: "选择你要提交的类型 :",
			scope: "选择一个提交范围（可选）:",
			customScope: "请输入自定义的提交范围 :",
			subject: "填写简短精炼的变更描述 :\n",
			body: '填写更加详细的变更描述（可选）。使用 "|" 换行 :\n',
			breaking: '列举非兼容性重大的变更（可选）。使用 "|" 换行 :\n',
			footerPrefixesSelect: "选择关联issue前缀（可选）:",
			customFooterPrefix: "输入自定义issue前缀 :",
			footer: "列举关联issue (可选) 例如: closes #31, fixes #42, refs #123 :\n",
			generatingByAI: "正在通过 AI 生成你的提交简短描述...",
			generatedSelectByAI: "选择一个 AI 生成的简短描述:",
			confirmCommit: "是否提交或修改commit ?",
		},
		types: [
			{
				value: "feat",
				name: "feat:     ✨ 新增功能 | A new feature",
				emoji: ":sparkles:",
			},
			{
				value: "fix",
				name: "fix:      🐛 修复缺陷 | A bug fix",
				emoji: ":bug:",
			},
			{
				value: "docs",
				name: "docs:     📝 文档更新 | Documentation only changes",
				emoji: ":memo:",
			},
			{
				value: "style",
				name: "style:    💄 代码格式 | Changes that do not affect the meaning of the code",
				emoji: ":lipstick:",
			},
			{
				value: "refactor",
				name: "refactor: ♻️ 代码重构 | A code change that neither fixes a bug nor adds a feature",
				emoji: ":recycle:",
			},
			{
				value: "perf",
				name: "perf:     ⚡️ 性能提升 | A code change that improves performance",
				emoji: ":zap:",
			},
			{
				value: "test",
				name: "test:     ✅ 测试相关 | Adding missing tests or correcting existing tests",
				emoji: ":white_check_mark:",
			},
			{
				value: "build",
				name: "build:    📦️ 构建相关 | Changes that affect the build system or external dependencies",
				emoji: ":package:",
			},
			{
				value: "ci",
				name: "ci:       🎡 持续集成 | Changes to our CI configuration files and scripts",
				emoji: ":ferris_wheel:",
			},
			{
				value: "revert",
				name: "revert:   ⏪️ 回退代码 | Revert to a commit",
				emoji: ":rewind:",
			},
			{
				value: "chore",
				name: "chore:    🔨 其他修改 | Other changes that do not modify src or test files",
				emoji: ":hammer:",
			},
		],
		useEmoji: true,
		emojiAlign: "center",
		useAI: false,
		aiNumber: 1,
		themeColorCode: "",
		// scope 范围定义，基于 DevEnvLite 项目架构 (Tauri + Vue3 + Rust)
		scopes: [
			"ui", // 前端界面：Vue 组件、样式、交互逻辑
			"core", // 核心逻辑：环境变量处理、业务逻辑、数据库操作
			"tauri", // Tauri 相关：命令接口、配置、原生 API 调用
			"i18n", // 国际化：多语言支持、本地化
			"config", // 配置管理：应用配置、构建配置、环境配置
			"docs", // 文档相关：README、模板、注释
			"build", // 构建部署：打包、发布、CI/CD
		],
		enableMultipleScopes: true,
		scopeEnumSeparator: ",",
		allowCustomScopes: true,
		allowEmptyScopes: true,
		customScopesAlign: "bottom",
		customScopesAlias: "custom",
		emptyScopesAlias: "empty",
		upperCaseSubject: false,
		markBreakingChangeMode: false,
		allowBreakingChanges: ["feat", "fix"],
		breaklineNumber: 100,
		breaklineChar: "|",
		skipQuestions: [],
		// issue 前缀配置，精简版
		issuePrefixes: [
			{ value: "fixes", name: "fixes:     🐛 修复问题" },
			{ value: "fix", name: "fix:       🐛 修复问题" },
			{ value: "fixed", name: "fixed:     🐛 修复问题" },
			{ value: "closes", name: "closes:    ✅ 关闭问题" },
			{ value: "closed", name: "closed:    ✅ 关闭问题" },
			{ value: "close", name: "close:     ✅ 关闭问题" },
			{ value: "refs", name: "refs:      📎 引用问题" },
			{ vlaue: "reopen", name: "reopen:    ❌ 重新打开问题" },
		],
		customIssuePrefixAlign: "top",
		emptyIssuePrefixAlias: "skip",
		customIssuePrefixAlias: "custom",
		allowCustomIssuePrefix: true,
		allowEmptyIssuePrefix: true,
		confirmColorize: true,
		maxHeaderLength: Number.POSITIVE_INFINITY,
		maxSubjectLength: Number.POSITIVE_INFINITY,
		minSubjectLength: 0,
		scopeOverrides: undefined,
		defaultBody: "",
		defaultIssues: "",
		defaultScope: "",
		defaultSubject: "",
	},
};
