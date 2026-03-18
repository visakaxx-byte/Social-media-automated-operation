# Social Media Automation System

[![Rust](https://img.shields.io/badge/Rust-1.94+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Development Status](https://img.shields.io/badge/Status-Phase%205%20Complete-green.svg)](https://github.com/visakaxx-byte/Social-media-automated-operation)

社交媒体自动运营系统 - 基于 Rust + Chromiumoxide + CDP 的本地自动化工具

> **A robust social media automation system built with Rust, featuring multi-platform support, browser automation with anti-detection, task scheduling, and a modern web interface.**

## ✨ 功能特性

- ✅ **多平台支持**: 小红书、Twitter（可扩展更多平台）
- ✅ **浏览器自动化**: 基于 Chromiumoxide (CDP) 的浏览器控制
- ✅ **反检测机制**: 完善的 Stealth Patch 和指纹伪装
- ✅ **人类行为模拟**: 贝塞尔曲线鼠标轨迹、随机输入延迟、人类化滚动
- ✅ **任务调度系统**: 异步任务处理、定时任务、重试机制
- ✅ **账号管理**: 多账号支持、健康度追踪、状态管理
- ✅ **数据持久化**: SQLite 数据库 + YAML 配置
- ✅ **Web 管理界面**: REST API + WebSocket 实时通信
- ✅ **CLI 工具**: 完整的命令行界面

## 📁 项目结构

```
social-auto/
├── src/
│   ├── browser/          # 浏览器自动化引擎
│   │   ├── engine.rs     # Chromiumoxide 封装
│   │   ├── stealth.rs    # Stealth patch
│   │   └── behavior.rs   # 人类行为模拟
│   ├── platforms/        # 平台适配器
│   │   ├── platform_trait.rs  # 统一接口
│   │   ├── xiaohongshu.rs     # 小红书适配器
│   │   └── twitter.rs         # Twitter 适配器
│   ├── core/             # 核心业务逻辑
│   │   ├── scheduler.rs  # 任务调度器
│   │   ├── account_manager.rs  # 账号管理器
│   │   └── executor.rs   # 任务执行器
│   ├── web/              # Web 服务
│   │   ├── server.rs     # Axum 服务器
│   │   ├── routes/       # API 路由
│   │   └── state.rs      # 状态管理
│   ├── cli/              # CLI 接口
│   ├── config/           # 配置管理
│   ├── db/               # 数据库层
│   ├── models/           # 数据模型
│   └── utils/            # 工具函数
├── web-ui/               # Web 前端
│   ├── index.html
│   ├── app.js
│   └── styles.css
├── config.yaml           # 主配置文件
└── data/                 # 数据目录
```

## 🚀 快速开始

详细使用指南请查看 [QUICK_START.md](QUICK_START.md)

### 1. 安装依赖

确保已安装 Rust 工具链和 Chrome/Chromium 浏览器：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# macOS 安装 Chrome
brew install --cask google-chrome
```

### 2. 构建项目

```bash
cargo build --release
```

### 3. 启动 Web 界面

```bash
cargo run --release -- web --port 8080
```

然后在浏览器中打开 `http://localhost:8080`

### 4. 使用 CLI

#### 账号管理

```bash
# 添加账号
cargo run -- account add --platform xiaohongshu --username user123

# 列出所有账号
cargo run -- account list

# 查看账号状态
cargo run -- account status <ACCOUNT_ID>
```

#### 任务管理

```bash
# 创建发布任务
cargo run -- task create --task-type post --account <ACCOUNT_ID> --content <CONTENT_ID>

# 列出所有任务
cargo run -- task list

# 启动调度器
cargo run -- start
```

#### 内容管理

```bash
# 添加内容
echo "这是一条测试内容 #测试" > post.txt
cargo run -- content add --file post.txt --platform xiaohongshu

# 列出所有内容
cargo run -- content list
```

## 🛠️ 技术栈

### 后端
- **语言**: Rust 1.94+
- **异步运行时**: Tokio 1.35
- **Web 框架**: Axum 0.7
- **浏览器自动化**: Chromiumoxide 0.5
- **数据库**: SQLite (rusqlite 0.30)
- **配置**: YAML (serde_yaml)
- **CLI**: Clap 4.4
- **加密**: AES-256-GCM

### 前端
- **HTML5** + **CSS3** + **JavaScript (ES6+)**
- **WebSocket API** - 实时通信
- **Fetch API** - HTTP 请求

## 🔒 反检测特性

### Stealth Patch

- ✅ Canvas 指纹随机化
- ✅ WebGL 指纹伪装
- ✅ WebRTC IP 泄露防护
- ✅ Navigator 属性伪装（userAgent、platform、hardwareConcurrency）
- ✅ 时区和语言设置
- ✅ 屏幕分辨率随机化

### 行为模拟

- ✅ 贝塞尔曲线鼠标轨迹
- ✅ 随机输入延迟（模拟真实打字速度）
- ✅ 人类化滚动（随机速度和停顿）
- ✅ 随机停顿时间（避免机器特征）

## 📚 文档

- [快速开始指南](QUICK_START.md) - 5 分钟快速上手
- [编译和测试指南](COMPILE_AND_TEST.md) - 详细的编译和测试说明
- [测试报告](TEST_REPORT.md) - 功能测试结果
- [开发进度报告](PROGRESS_REPORT.md) - Phase 1-4 开发记录
- [最终完整报告](FINAL_REPORT.md) - 项目完整开发报告
- [设计文档](docs/superpowers/specs/2026-03-17-social-media-automation-design.md) - 系统架构设计

## 📊 开发状态

| Phase | 功能模块 | 状态 | 完成时间 |
|-------|---------|------|---------|
| Phase 1 | 基础框架（配置、数据模型、数据库、CLI） | ✅ 完成 | 2026-03-17 |
| Phase 2 | 浏览器引擎（Chromiumoxide + Stealth Patch） | ✅ 完成 | 2026-03-17 |
| Phase 3 | 平台适配器（小红书、Twitter） | ✅ 完成 | 2026-03-18 |
| Phase 4 | 核心业务逻辑（任务调度、账号管理、执行器） | ✅ 完成 | 2026-03-18 |
| Phase 5 | Web 界面（REST API + WebSocket + 前端） | ✅ 完成 | 2026-03-18 |
| Phase 6 | 测试与优化（单元测试、集成测试、性能优化） | 🚧 进行中 | - |

**当前版本**: v1.0.0
**代码行数**: 5,000+ 行
**模块数量**: 30+ 个
**API 端点**: 15+ 个

### 已实现功能

- ✅ 完整的 CLI 命令行工具
- ✅ Web 管理界面（仪表盘、账号管理、任务管理、内容库）
- ✅ 实时 WebSocket 通信
- ✅ 任务调度系统（异步处理、定时任务、重试机制）
- ✅ 账号管理系统（缓存、健康度评分）
- ✅ 浏览器自动化引擎（反检测、行为模拟）
- ✅ 平台适配器框架（可扩展设计）

### 待完成功能

- 🚧 完善的单元测试和集成测试
- 🚧 性能优化和安全加固
- 📋 更多平台适配器实现
- 📋 AI 内容生成集成
- 📋 数据分析和报表功能

## ⚠️ 安全提示

1. **合法使用**: 本工具仅供学习和研究使用
2. **遵守条款**: 请遵守各平台的服务条款和使用政策
3. **隐私保护**: 建议使用代理 IP 以保护隐私
4. **数据安全**: 账号密码使用 AES-256-GCM 加密存储
5. **频率控制**: 合理设置操作频率，避免触发平台风控
6. **账号安全**: 建议使用小号测试，避免主账号风险

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

在提交 PR 之前，请确保：
- 代码通过 `cargo fmt` 格式化
- 代码通过 `cargo clippy` 检查
- 添加必要的测试
- 更新相关文档

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🔗 相关链接

- [GitHub Repository](https://github.com/visakaxx-byte/Social-media-automated-operation)
- [Issue Tracker](https://github.com/visakaxx-byte/Social-media-automated-operation/issues)
- [Chromiumoxide Documentation](https://docs.rs/chromiumoxide/)
- [Rust Documentation](https://doc.rust-lang.org/)

---

**开发状态**: 🟢 Phase 5 完成，Phase 6 进行中
**维护状态**: 🟢 持续维护
**最后更新**: 2026-03-18
