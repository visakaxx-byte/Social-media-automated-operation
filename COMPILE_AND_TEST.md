# 编译和测试指南

## 1. 安装 Rust

如果还没有安装 Rust，请运行：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，重新加载环境变量：

```bash
source $HOME/.cargo/env
```

验证安装：

```bash
rustc --version
cargo --version
```

## 2. 编译项目

### 开发模式编译（快速，包含调试信息）

```bash
cd /Users/gary/Desktop/社媒自动运营
cargo build
```

### 发布模式编译（优化，速度更快）

```bash
cargo build --release
```

## 3. 测试基本功能

### 3.1 测试账号管理

```bash
# 添加账号
cargo run -- account add --platform xiaohongshu --username test_user

# 列出所有账号
cargo run -- account list

# 查看账号状态（使用上一步返回的账号 ID）
cargo run -- account status <ACCOUNT_ID>
```

### 3.2 测试内容管理

```bash
# 创建测试内容文件
echo "这是一条测试内容" > test_content.txt

# 添加内容
cargo run -- content add --file test_content.txt --platform xiaohongshu

# 列出所有内容
cargo run -- content list
```

### 3.3 测试任务管理

```bash
# 创建任务（需要先有账号和内容）
cargo run -- task create --task-type post --account <ACCOUNT_ID> --content <CONTENT_ID>

# 列出所有任务
cargo run -- task list

# 列出特定状态的任务
cargo run -- task list --status pending
```

## 4. 可能遇到的问题

### 问题 1: 编译错误

如果遇到依赖相关的编译错误，可能需要更新依赖版本。检查错误信息并根据提示调整 `Cargo.toml`。

### 问题 2: Chrome/Chromium 未找到

浏览器引擎需要 Chrome 或 Chromium。如果系统没有安装：

**macOS:**
```bash
brew install --cask google-chrome
# 或
brew install chromium
```

**Linux:**
```bash
sudo apt install chromium-browser
# 或
sudo apt install google-chrome-stable
```

### 问题 3: 数据库权限

确保 `data/` 目录有写入权限：

```bash
mkdir -p data
chmod 755 data
```

## 5. 检查编译输出

编译成功后，可执行文件位置：

- 开发模式: `target/debug/social-auto`
- 发布模式: `target/release/social-auto`

直接运行：

```bash
# 开发模式
./target/debug/social-auto --help

# 发布模式
./target/release/social-auto --help
```

## 6. 查看日志

设置日志级别：

```bash
# 详细日志
RUST_LOG=debug cargo run -- account list

# 仅错误
RUST_LOG=error cargo run -- account list
```

## 7. 运行单元测试

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test crypto

# 显示测试输出
cargo test -- --nocapture
```

## 8. 代码检查

```bash
# 检查代码（不编译）
cargo check

# 代码格式化
cargo fmt

# 代码 lint
cargo clippy
```

## 9. 清理构建

如果需要重新编译：

```bash
cargo clean
cargo build
```

## 10. 下一步开发

编译和基本测试通过后，可以继续开发：

- **Phase 3**: 平台适配器（小红书、Twitter 等）
- **Phase 4**: 任务调度器和执行引擎
- **Phase 5**: Web 管理界面

## 注意事项

1. 首次编译会下载所有依赖，可能需要较长时间
2. 确保网络连接正常（需要访问 crates.io）
3. 如果在国内，建议配置 Cargo 镜像源加速下载
4. 浏览器自动化功能需要实际的 Chrome/Chromium 才能测试

## Cargo 镜像配置（可选，加速下载）

创建或编辑 `~/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

或使用字节跳动镜像：

```toml
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
```
