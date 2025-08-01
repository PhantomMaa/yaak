# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此代码库中工作时提供指导。

## 项目概述

Yaak 是一个桌面 API 客户端，支持 REST、GraphQL、SSE、WebSocket 和 gRPC API。它使用 Tauri（Rust 后端）和 React（TypeScript 前端）构建，采用单仓库结构，具有扩展的插件系统。

## 开发命令

**安装和开发:**
```bash
npm install                    # 安装依赖
npm run bootstrap             # 初始设置（vendor Node.js、插件、protoc）
npm start                     # 启动开发应用（app-dev 别名）
npm run app-dev               # 启动 Tauri 开发模式
```

**构建:**
```bash
npm run build                 # 构建所有工作空间
npm run app-build             # 构建生产版 Tauri 应用
npm run build-plugins         # 构建所有插件
```

**代码质量:**
```bash
npm run lint                  # 在所有工作空间运行 ESLint
```

**数据库:**
```bash
npm run migration             # 创建新的 SQLite 迁移
```

## 架构概述

### 核心结构
- **src-tauri/**: Rust 后端，包含模块化 crate（yaak-models、yaak-http、yaak-grpc 等）
- **src-web/**: React 前端，使用 TypeScript，Vite 打包
- **plugins/**: 可扩展插件系统，支持认证、导入器、模板函数等
- **packages/**: 共享库（plugin-runtime、common-lib）

### 核心技术
- **后端**: Tauri 2.x、Rust、SQLite 带迁移
- **前端**: React 19、TypeScript、TanStack Router/Query、Tailwind CSS、Jotai 状态管理
- **插件**: Node.js 运行时，通过 gRPC 与主应用通信
- **编辑器**: CodeMirror 6，带自定义语言解析器（Twig 模板、URL 参数）

### 插件系统
Yaak 具有复杂的插件架构，支持：
- 认证插件（OAuth2、JWT、Basic 等）
- 模板函数（UUID、hash、timestamp 等）
- 导入器（Postman、Insomnia、OpenAPI、cURL）
- 操作和过滤器

每个插件都是独立的工作空间，有自己的构建过程。

### 数据库模型
使用 SQLite，在 `src-tauri/yaak-models/migrations/` 中有完整的迁移。主要模型：
- 工作空间、环境、文件夹
- HTTP/gRPC/WebSocket 请求和响应
- 认证、cookie、变量
- 插件数据和设置

### 状态管理
- **Jotai** 用于 React 状态，采用 atoms 模式
- **TanStack Query** 用于服务器状态和缓存
- `src-web/hooks/` 中的自定义 hooks 处理业务逻辑

### 关键目录
- `src-web/components/`: UI 组件（核心组件在 `core/` 子目录中）
- `src-web/hooks/`: React hooks 处理业务逻辑
- `src-web/lib/`: 工具函数和共享逻辑
- `src-tauri/src/`: 主要 Rust 应用代码
- `src-tauri/yaak-*/`: 特定功能的模块化 Rust crate

## 开发说明

### 前端开发
React 应用使用现代模式：
- 基于文件的路由，使用 TanStack Router
- 组件组合，使用 render props 和自定义 hooks
- Tailwind 样式，带自定义主题系统
- CodeMirror 6 高级文本编辑

### 后端开发
Rust 后端高度模块化：
- 每个主要功能都是独立的 crate（yaak-models、yaak-http 等）
- Tauri 命令将 Rust 功能暴露给前端
- 数据库操作使用自定义查询管理器和 SQLx

### 插件开发
插件在独立的 Node.js 进程中运行：
- TypeScript 插件，构建为 JavaScript
- 通过 gRPC 与主应用通信
- 插件运行时提供 HTTP 请求、存储等 API

### 测试
- 单个插件有自己的测试套件
- 配置了 Vitest 进行测试
- 没有全局测试命令 - 每个工作空间运行测试

代码库遵循单仓库模式，每个工作空间（插件、包、主应用）可以独立开发和构建，同时共享通用依赖和工具。