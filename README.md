# Scoop UI

<div align="center">

一个现代化的 [Scoop](https://scoop.sh/) 包管理器图形界面工具

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)

</div>

## 📖 简介

Scoop UI 是一个基于 Tauri + SvelteKit 构建的桌面应用程序,为 Windows 包管理器 Scoop 提供了直观、美观的图形界面。通过这个工具,您可以轻松地浏览、搜索、安装和管理 Scoop 应用,无需使用命令行。

## ✨ 功能特性

### 🎯 核心功能

- **应用发现** - 浏览精选应用和随机推荐,发现新工具
- **应用搜索** - 快速搜索本地和远程 Scoop 应用仓库
- **应用管理** - 查看已安装应用,显示版本、大小和更新状态
- **一键安装** - 通过图形界面轻松安装应用,实时显示安装进度
- **应用更新** - 支持单个应用更新和批量更新所有应用
- **Bucket 管理** - 添加、删除和查看 Scoop Buckets
- **应用详情** - 查看应用的详细信息,包括描述、版本、许可证、依赖等

### 🎨 界面特性

- **现代化设计** - 采用 Material Design 风格,界面简洁美观
- **深色模式** - 支持浅色/深色主题切换
- **响应式布局** - 适配不同窗口大小
- **实时进度** - 安装和更新操作显示实时进度反馈
- **流畅动画** - 平滑的过渡动画和交互效果

## 🛠️ 技术栈

### 前端
- **[SvelteKit](https://kit.svelte.dev/)** - 现代化的 Web 应用框架
- **[Svelte 5](https://svelte.dev/)** - 响应式 UI 框架
- **TypeScript** - 类型安全的 JavaScript
- **Vite** - 快速的构建工具

### 后端
- **[Tauri 2](https://tauri.app/)** - 轻量级桌面应用框架
- **Rust** - 高性能系统编程语言
- **PowerShell** - 与 Scoop 命令行工具交互

### 依赖
- `@tauri-apps/api` - Tauri API 绑定
- `@tauri-apps/plugin-shell` - Shell 命令执行插件
- `@tauri-apps/plugin-opener` - 文件/URL 打开插件

## 📦 安装

### 前置要求

1. **安装 Scoop**
   ```powershell
   # 在 PowerShell 中运行
   Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
   irm get.scoop.sh | iex
   ```

2. **安装 Node.js** (推荐使用 Bun)
   ```powershell
   scoop install nodejs
   # 或安装 Bun
   scoop install bun
   ```

3. **安装 Rust**
   ```powershell
   scoop install rustup
   rustup default stable
   ```

### 开发环境设置

1. **克隆仓库**
   ```bash
   git clone <repository-url>
   cd scoop-ui
   ```

2. **安装依赖**
   ```bash
   # 使用 npm
   npm install
   
   # 或使用 Bun (更快)
   bun install
   ```

3. **运行开发服务器**
   ```bash
   # 使用 npm
   npm run tauri dev
   
   # 或使用 Bun
   bun run tauri dev
   ```

### 构建生产版本

```bash
# 使用 npm
npm run tauri build

# 或使用 Bun
bun run tauri build
```

构建完成后,可执行文件将位于 `src-tauri/target/release/` 目录中。

## 🚀 使用指南

### 主要页面

1. **发现页面** (`/`)
   - 浏览编辑推荐应用
   - 查看随机推荐的工具
   - 浏览热门生产力工具和最新更新

2. **已安装** (`/installed`)
   - 查看所有已安装的应用
   - 检查可用更新
   - 批量更新或单独更新应用
   - 卸载不需要的应用

3. **搜索** (`/search`)
   - 搜索本地和远程应用
   - 查看应用详细信息
   - 快速安装应用

4. **Buckets** (`/buckets`)
   - 管理 Scoop Buckets
   - 添加新的软件源
   - 删除不需要的 Buckets

5. **更新** (`/updates`)
   - 查看所有可更新的应用
   - 一键更新所有应用
   - 更新 Scoop 本身

### 常用操作

- **安装应用**: 在发现或搜索页面点击应用卡片,然后点击"安装"按钮
- **更新应用**: 在已安装页面点击应用旁的"更新"按钮
- **批量更新**: 在更新页面点击"全部更新"按钮
- **添加 Bucket**: 在 Buckets 页面点击"添加 Bucket"按钮

## 📁 项目结构

```
scoop-ui/
├── src/                      # 前端源代码
│   ├── lib/                  # 库文件
│   │   ├── components/       # Svelte 组件
│   │   ├── scoop.ts         # Scoop API 封装
│   │   └── stores.ts        # 状态管理
│   ├── routes/              # 页面路由
│   │   ├── +page.svelte     # 发现页面
│   │   ├── installed/       # 已安装页面
│   │   ├── search/          # 搜索页面
│   │   ├── buckets/         # Buckets 页面
│   │   └── updates/         # 更新页面
│   └── app.css              # 全局样式
├── src-tauri/               # Tauri 后端
│   ├── src/
│   │   ├── lib.rs           # 主要 Rust 逻辑
│   │   ├── install.rs       # 安装功能
│   │   └── main.rs          # 入口点
│   ├── Cargo.toml           # Rust 依赖
│   └── tauri.conf.json      # Tauri 配置
├── static/                  # 静态资源
├── package.json             # Node.js 依赖
└── vite.config.js          # Vite 配置
```

## 🔧 开发

### 推荐的 IDE 设置

- **[VS Code](https://code.visualstudio.com/)**
- **扩展**:
  - [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 可用脚本

```bash
# 开发模式
npm run dev          # 运行前端开发服务器
npm run tauri dev    # 运行 Tauri 开发模式

# 构建
npm run build        # 构建前端
npm run tauri build  # 构建完整应用

# 代码检查
npm run check        # 运行 Svelte 类型检查
npm run check:watch  # 监听模式下的类型检查
```

## 🤝 贡献

欢迎贡献代码、报告问题或提出新功能建议!

## 📄 许可证

本项目采用 [MIT](LICENSE) 许可证。

## 🙏 致谢

- [Scoop](https://scoop.sh/) - 优秀的 Windows 包管理器
- [Tauri](https://tauri.app/) - 轻量级桌面应用框架
- [SvelteKit](https://kit.svelte.dev/) - 现代化的 Web 应用框架
