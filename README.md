<div align="center" id="top">
  <img src="./.github/logo.ico" alt="devEnvLite" />

  &#xa0;

  <!-- <a href="https://devEnvLite.netlify.app">Demo</a> -->
</div>

<h1 align="center">DevEnvLite</h1>

<p align="center">
  <a href="https://github.com/ddki/devEnvLite/releases">
    <img alt="Github release" src="https://img.shields.io/github/release/ddki/devEnvLite">
  </a>

  <a href="#">
    <img alt="Github top language" src="https://img.shields.io/github/languages/top/ddki/devEnvLite">
  </a>

  <a href="#">
    <img alt="Github language count" src="https://img.shields.io/github/languages/count/ddki/devEnvLite">
  </a>

  <a href="#">
    <img alt="Repository size" src="https://img.shields.io/github/repo-size/ddki/devEnvLite">
  </a>

  <a href="https://github.com/ddki/devEnvLite/blob/master/LICENSE">
    <img alt="License" src="https://img.shields.io/github/license/ddki/devEnvLite">
  </a>

  <a href="https://github.com/ddki/devEnvLite/issues">
    <img alt="Github issues" src="https://img.shields.io/github/issues/ddki/devEnvLite" />
  </a>

  <a href="https://github.com/ddki/devEnvLite/forks">
    <img alt="Github forks" src="https://img.shields.io/github/forks/ddki/devEnvLite" />
  </a>

  <a href="https://github.com/ddki/devEnvLite/stargazers">
    <img alt="Github stars" src="https://img.shields.io/github/stars/ddki/devEnvLite" />
  </a>
</p>

<!-- Status -->

<h4 align="center"> 
	一个环境变量管理和配置工具。 A lightweight tool for managing and configuring environment variables. 
</h4> 

<hr>

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0;
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#rocket-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/ddki" target="_blank">Author</a>
</p>

<br>

## :dart: About ##

:man_technologist: 一个环境变量管理和配置工具。 A lightweight tool for managing and configuring environment variables. 

## :sparkles: Features ##

- support multiple platforms
  - :white_check_mark: windows
  - :white_large_square: Linux
  - :white_large_square: Mac

- :white_check_mark: Application settings
	- :white_check_mark: multiple languages
	- :white_check_mark: application directories

- :white_check_mark: Collate your environment variables;
- :white_large_square: Backup your environment variables to file;
- :white_large_square: Recover your environment variables from backup file;

- :white_large_square: Manage your configurations for different environment variables
	- :white_check_mark: create, modify, delete, active, check, apply... your configuration, part done
	- :white_check_mark: import configuration from system environment variables
	- :white_large_square: import configuration from file
	- :white_large_square: import configuration from url

- :white_large_square: Manage group of the configuration;
	- :white_check_mark: part done

- :white_large_square: Manage env of the configuration's group;
	- :white_check_mark: part done


- 多平台支持
  - :white_check_mark: windows
  - :white_large_square: Linux
  - :white_large_square: Mac

- :white_check_mark: 设置
	- :white_check_mark: 多语言支持（中、英）
	- :white_check_mark: 其他目录配置
	- :white_large_square: 更新

- :white_check_mark: 一键整理环境变量
- :white_large_square: 备份环境变量
- :white_large_square: 从备份恢复环境变量

- :white_large_square: 管理不同配置
	- :white_check_mark: 增删改查、激活配置
	- :white_large_square: 应用
	- :white_large_square: 检查应用状态
	- :white_check_mark: 从当前系统环境变量导入配置
	- :white_large_square: 从文件导入配置
	- :white_large_square: 从网络导入配置

- :white_large_square: 管理配置分组
	- :white_check_mark: 增删改查
	- :white_large_square: 应用
	- :white_large_square: 检查应用状态

- :white_large_square: 管理配置分组中的环境变量
	- :white_check_mark: 增删改查、复制
	- :white_large_square: 应用

## :rocket: Technologies

The following tools were used in this project:

**font:**
- [Tauri2](https://beta.tauri.app/)
- [Vite](https://vitejs.dev/)
- [Vue3](https://vuejs.org/)
- [tailwindcss](https://tailwindcss.com/)
- [shadcn-vue](https://www.shadcn-vue.com)
- [lucide-vue-next](https://lucide.dev)
- [Node.js](https://nodejs.org/en/)
- [TypeScript](https://www.typescriptlang.org/)
- [cz-git](https://cz-git.qbb.sh)
- [release-it](https://github.com/release-it/release-it)
- [husky](https://github.com/husky/husky)
- [biome](https://biomejs.dev/)
- ...

**rust:**
- [serde](https://github.com/serde-rs/serde)
- [thiserror](https://github.com/dtolnay/thiserror)
- [anyhow](https://github.com/dtolnay/anyhow)
- [winreg](https://github.com/gentoo90/winreg-rs)
- [log](https://github.com/rust-lang/log)

Thanks to these contributors.

### Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

### Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
   1. Run `Extensions: Show Built-in Extensions` from VSCode's command palette
   2. Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

### Tauri updater with [Tauri](https://next--tauri.netlify.app/next/guides/distribution/updater)

#### :ferris_wheel: Setting for Github

1. Project -> Settings
2. Security -> Secrets and Variables -> Actions
3. Secrets -> new repository secret

```sh
TAURI_KEY_PASSWORD="your password"
TAURI_PRIVATE_KEY="your private key"
```

### About Tauri Version

2.0 预览版本随时都在改变，项目中的这个版本是测试通过的，不要轻易去改变 tauri 的版本，等待 2.0 正式版发布。

### Upgrade project's rust dependencies

[cargo-edit](https://github.com/killercup/cargo-edit) can upgrade project dependencies to lastest version.

```bash
# install
cargo install cargo-edit

# upgrade
cargo upgrade
```

## :white_check_mark: Requirements

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com), [Node](https://nodejs.org/en/), [Rust](https://www.rust-lang.org/) installed.

## :checkered_flag: Starting

```bash
# Clone this project
git clone https://github.com/ddki/devEnvLite

# Access
cd devEnvLite

# Install dependencies
pnpm install

# Run the project
pnpm tauri dev

# build the project
pnpm tauri build

# commit file to git
git add --all
pnpm commit

# release
pnpm release
```

## Release Step

```bash
# step 1
git add -A
# step 2
pnpm commit
# step 3
pnpm release
```

## :memo: License ##

This project is under license from GPL3.0. For more details, see the [LICENSE](LICENSE) file.

Made with :heart: by <a href="https://github.com/ddki" target="_blank">ddki</a>

&#xa0;

<a href="#top">Back to top</a>
