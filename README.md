# Social Media Automation System

社交媒体自动运营系统 - 基于 Rust + Chromey + CDP 的本地自动化工具

## 功能特性

- ✅ **多平台支持**: 小红书、抖音、微博、Twitter、Instagram 等
- ✅ **浏览器自动化**: 基于 Chromiumoxide (CDP) 的浏览器控制
- ✅ **反检测机制**: 完善的 Stealth Patch 和指纹伪装
- ✅ **人类行为模拟**: 鼠标轨迹、输入延迟、随机滚动
- ✅ **数据持久化**: SQLite 数据库 + YAML 配置
- ✅ **CLI 界面**: 完整的命令行工具

## 项目结构

```
social-auto/
├── src/
│   ├── browser/        # 浏览器自动化引擎
│   │   ├── engine.rs   # Chromey 封装
│   │   ├── stealth.rs  # Stealth patch
│   │   └── behavior.rs # 人类行为模拟
│   ├── cli/            # CLI 接口
│   ├── config/         # 配置管理
│   ├── db/             # 数据库层
│   ├── models/         # 数据模型
│   └── utils/          # 工具函数
├── config.yaml         # 主配置文件
└── data/               # 数据目录
```

## 快速开始

### 1. 安装依赖

确保已安装 Rust 工具链：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 构建项目

```bash
cargo build --release
```

### 3. 配置

编辑 `config.yaml` 文件，配置系统参数、浏览器设置、反检测选项等。

### 4. 使用

#### 账号管理

```bash
# 添加账号
./target/release/social-auto account add --platform xiaohongshu --username user123

# 列出所有账号
./target/release/social-auto account list

# 查看账号状态
./target/release/social-auto account status <ACCOUNT_ID>

# 删除账号
./target/release/social-auto account remove <ACCOUNT_ID>
```

#### 任务管理

```bash
# 创建发布任务
./target/release/social-auto task create --task-type post --account <ACCOUNT_ID> --content <CONTENT_ID>

# 列出所有任务
./target/release/social-auto task list

# 取消任务
./target/release/social-auto task cancel <TASK_ID>
```

#### 内容管理

```bash
# 添加内容
./target/release/social-auto content add --file ./post.txt --platform xiaohongshu

# 列出所有内容
./target/release/social-auto content list

# 删除内容
./target/release/social-auto content remove <CONTENT_ID>
```

## 技术栈

- **语言**: Rust
- **浏览器自动化**: chromiumoxide (CDP)
- **异步运行时**: Tokio
- **数据库**: SQLite (rusqlite)
- **配置**: YAML (serde_yaml)
- **CLI**: clap
- **加密**: AES-256-GCM

## 反检测特性

### Stealth Patch

- Canvas 指纹随机化
- WebGL 指纹伪装
- WebRTC IP 泄露防护
- Navigator 属性伪装
- 时区和语言设置
- 屏幕分辨率随机化

### 行为模拟

- 贝塞尔曲线鼠标轨迹
- 随机输入延迟
- 人类化滚动
- 随机停顿时间

## 开发状态

当前版本实现了基础框架和浏览器引擎核心功能：

- ✅ Phase 1: 基础框架
- ✅ Phase 2: 浏览器引擎
- ⏳ Phase 3: 平台适配器
- ⏳ Phase 4: 任务调度器
- ⏳ Phase 5: Web 界面

## 安全提示

1. 本工具仅供学习和研究使用
2. 请遵守各平台的服务条款
3. 建议使用代理 IP 以保护隐私
4. 账号密码使用 AES-256 加密存储
5. 合理设置操作频率，避免触发平台限流

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
