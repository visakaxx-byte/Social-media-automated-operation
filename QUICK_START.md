# 快速开始指南

## 🚀 5 分钟快速上手

### 1. 启动 Web 界面

```bash
cargo run --release -- web --port 8080
```

在浏览器中打开: `http://localhost:8080`

### 2. 添加账号

在 Web 界面中：
1. 点击"账号管理"标签
2. 点击"+ 添加账号"按钮
3. 选择平台（小红书/Twitter）
4. 输入用户名
5. 点击"添加"

或使用 CLI：
```bash
cargo run -- account add --platform xiaohongshu --username myuser
```

### 3. 添加内容

```bash
echo "这是一条测试内容 #测试" > post.txt
cargo run -- content add --file post.txt --platform xiaohongshu
```

### 4. 创建任务

在 Web 界面中：
1. 点击"任务管理"标签
2. 点击"+ 创建任务"按钮
3. 选择任务类型
4. 选择账号和内容
5. 点击"创建"

或使用 CLI：
```bash
cargo run -- task create --task-type post --account <ACCOUNT_ID> --content <CONTENT_ID>
```

### 5. 查看状态

```bash
cargo run -- status
```

或在 Web 界面的仪表盘查看实时统计。

---

## 📱 Web 界面功能

### 仪表盘
- 实时统计数据
- 账号/任务/内容概览
- 系统状态监控
- 日志流

### 账号管理
- 添加新账号
- 查看账号列表
- 查看账号状态和健康度
- 删除账号

### 任务管理
- 创建新任务
- 查看任务列表
- 查看任务状态
- 取消待执行任务

### 内容库
- 添加新内容
- 查看内容列表
- 查看使用统计
- 删除内容

---

## 🔧 配置说明

编辑 `config.yaml` 文件：

```yaml
# 系统配置
system:
  max_concurrent_tasks: 3    # 最大并发任务数
  task_timeout: 300          # 任务超时时间（秒）
  log_level: info            # 日志级别
  data_dir: "./data"         # 数据目录

# 浏览器配置
browser:
  headless: true             # 无头模式
  chrome_path: null          # Chrome 路径（null = 自动检测）
  user_data_dir: "./profiles"  # 用户数据目录
  window_size: [1920, 1080]  # 窗口大小

# 反检测配置
stealth:
  enabled: true              # 启用反检测
  randomize_fingerprint: true  # 随机化指纹
  block_webrtc: true         # 阻止 WebRTC
  timezone: "Asia/Shanghai"  # 时区

# 调度器配置
scheduler:
  check_interval: 60         # 检查间隔（秒）
  retry_max: 3               # 最大重试次数
  retry_backoff: [60, 300, 900]  # 重试延迟（秒）

# 频率限制
rate_limits:
  xiaohongshu:
    posts_per_day: 5         # 每天发布数
    likes_per_hour: 20       # 每小时点赞数
    comments_per_hour: 10    # 每小时评论数
    follows_per_day: 30      # 每天关注数
```

---

## 🎯 常见使用场景

### 场景 1: 定时发布内容

1. 添加账号
2. 准备内容
3. 创建定时任务
4. 启动调度器

```bash
# 启动调度器
cargo run -- start
```

### 场景 2: 批量互动

1. 添加多个账号
2. 创建点赞/评论任务
3. 系统自动执行

### 场景 3: 监控账号状态

使用 Web 界面实时查看：
- 账号健康度
- 任务执行情况
- 系统日志

---

## ⚠️ 注意事项

1. **首次使用**: 需要安装 Chrome/Chromium 浏览器
2. **账号安全**: 建议使用代理 IP
3. **操作频率**: 遵守平台限制，避免触发风控
4. **数据备份**: 定期备份 `data/database.db`
5. **日志查看**: 使用 `RUST_LOG=debug` 查看详细日志

---

## 🐛 故障排除

### 问题 1: 编译失败

```bash
# 清理并重新编译
cargo clean
cargo build --release
```

### 问题 2: 浏览器未找到

在 `config.yaml` 中指定 Chrome 路径：
```yaml
browser:
  chrome_path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
```

### 问题 3: 端口被占用

使用不同端口：
```bash
cargo run -- web --port 8081
```

### 问题 4: 数据库锁定

```bash
# 停止所有运行的实例
pkill social-auto

# 删除锁文件
rm data/database.db-*
```

---

## 📚 更多文档

- [COMPILE_AND_TEST.md](COMPILE_AND_TEST.md) - 编译和测试指南
- [TEST_REPORT.md](TEST_REPORT.md) - 测试报告
- [PROGRESS_REPORT.md](PROGRESS_REPORT.md) - 开发进度报告
- [FINAL_REPORT.md](FINAL_REPORT.md) - 最终完整报告

---

## 🎉 开始使用

现在你已经准备好开始使用社媒自动运营系统了！

```bash
# 启动 Web 界面
cargo run --release -- web --port 8080
```

访问 `http://localhost:8080` 开始你的自动化之旅！
