# 社媒自动运营系统设计文档

**日期：** 2026-03-17
**版本：** 1.0
**状态：** 设计阶段

## 1. 项目概述

### 1.1 目标

构建一个本地运行的社交媒体自动运营系统，支持国内外主流平台的内容发布和互动管理，具备高级反检测能力，适用于2-5个账号的运营需求。

### 1.2 核心功能

- **内容发布**：支持立即发布、定时发布、智能调度
- **互动管理**：自动点赞、智能评论、目标导流、社群维护
- **多平台支持**：国内（小红书、抖音、微博、B站）+ 国际（Twitter、Instagram、Facebook、LinkedIn）
- **反检测**：完善的浏览器指纹伪装和行为模拟
- **混合界面**：CLI命令行 + Web管理界面
- **混合存储**：SQLite数据库 + YAML配置文件

### 1.3 技术选型

- **核心语言**：Rust
- **浏览器自动化**：Chromey + CDP
- **Web框架**：Axum
- **异步运行时**：Tokio
- **数据库**：SQLite
- **配置管理**：YAML (serde_yaml)

## 2. 系统架构

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────┐
│                  用户交互层                          │
│  ┌──────────────┐         ┌──────────────┐         │
│  │  CLI 命令行  │         │  Web 管理界面 │         │
│  └──────┬───────┘         └──────┬───────┘         │
└─────────┼────────────────────────┼─────────────────┘
          │                        │
          └────────┬───────────────┘
                   │ HTTP/WebSocket
┌─────────────────┴─────────────────────────────────┐
│              Rust 核心服务层                       │
│  ┌──────────────────────────────────────────────┐ │
│  │  Axum Web Server (API + WebSocket)          │ │
│  └──────────────────────────────────────────────┘ │
│                                                    │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │任务调度器│  │账号管理器│  │内容管理器│       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
│       │             │             │              │
│  ┌────┴─────────────┴─────────────┴─────┐       │
│  │      浏览器自动化引擎 (Chromey)       │       │
│  │    + 自研 Stealth Patch 模块         │       │
│  └──────────────────────────────────────┘       │
└────────────────────┬───────────────────────────┘
                     │
┌────────────────────┴───────────────────────────┐
│              数据存储层                         │
│  ┌──────────────┐    ┌──────────────┐         │
│  │ SQLite 数据库 │    │ YAML 配置文件 │         │
│  │ (运行时数据)  │    │ (静态配置)    │         │
│  └──────────────┘    └──────────────┘         │
└────────────────────────────────────────────────┘
```

### 2.2 核心模块

1. **任务调度器** - 管理发布任务、互动任务的执行时机
2. **账号管理器** - 多账号切换、会话保持、指纹管理
3. **内容管理器** - 内容库、AI生成、内容适配
4. **浏览器引擎** - CDP控制、stealth patch、行为模拟
5. **Web服务** - REST API + WebSocket实时通信

## 3. 浏览器自动化引擎

### 3.1 Stealth Patch 模块（高优先级）

**指纹伪装：**
- Canvas指纹随机化
- WebGL指纹伪装
- 字体指纹混淆
- Audio Context指纹修改
- WebRTC IP泄露防护
- Navigator属性伪装（userAgent、platform、hardwareConcurrency等）
- 时区和语言设置
- 屏幕分辨率随机化

### 3.2 行为模拟

**人类化操作：**
- 鼠标轨迹模拟（贝塞尔曲线）
- 随机滚动和停顿
- 输入速度模拟（随机延迟）
- 页面停留时间随机化
- 操作间隔随机化（避免机器特征）

### 3.3 会话管理

- Cookie持久化存储
- LocalStorage/SessionStorage管理
- 浏览器配置文件隔离（每个账号独立）

## 4. 账号管理器

### 4.1 账号配置结构

```yaml
accounts:
  - id: account_001
    platform: xiaohongshu
    username: "user123"
    profile_path: "./profiles/account_001"
    proxy: "socks5://127.0.0.1:1080"
    fingerprint_preset: "preset_1"
    status: active
```

### 4.2 核心功能

- 账号池管理（增删改查）
- 登录状态检测和自动续期
- 账号健康度监控（异常检测）
- 代理IP轮换（可选）
- 账号冷却期管理（防止频繁操作）

## 5. 任务调度器

### 5.1 任务类型

- **PostTask** - 发布任务
- **InteractionTask** - 互动任务
- **ScheduledTask** - 定时任务
- **RecurringTask** - 循环任务

### 5.2 调度策略

- 优先级队列（紧急任务优先）
- 账号负载均衡（避免单账号过载）
- 时间窗口控制（最佳发布时间）
- 失败重试机制（指数退避）
- 并发控制（同时运行的任务数限制）

## 6. 内容管理器

### 6.1 内容源集成

- **本地内容库** - 文件系统存储（文本/图片/视频）
- **AI生成接口** - 对接OpenAI/Claude/本地模型
- **内容搬运** - 从其他平台抓取并改编
- **内容模板** - 支持变量替换和动态生成

### 6.2 内容适配

- 平台特定格式转换（字数限制、话题标签、@提及）
- 图片压缩和尺寸适配
- 视频转码（如需要）
- 敏感词过滤和替换

### 6.3 内容库结构

```
content/
├── posts/
│   ├── xiaohongshu/
│   ├── twitter/
│   └── instagram/
├── media/
│   ├── images/
│   └── videos/
└── templates/
    └── post_templates.yaml
```

## 7. 互动策略引擎

### 7.1 点赞策略

- 目标用户列表（竞品粉丝、相关话题）
- 每日配额限制（避免触发限流）
- 随机间隔（30秒-5分钟）
- 内容相关性过滤

### 7.2 评论策略

- **通用模板** - 预设评论库（"不错"、"学到了"等）
- **智能生成** - 基于帖子内容AI生成相关评论
- **关键词触发** - 特定关键词使用特定回复
- **长度控制** - 避免过短或过长

### 7.3 关注策略

- 目标用户筛选（粉丝数、活跃度）
- 关注/取关比例控制
- 互关检测和处理

### 7.4 社群维护

- 自动回复自己帖子下的评论
- 感谢点赞和关注
- 私信自动回复（可选）

## 8. 数据存储设计

### 8.1 数据库设计（SQLite）

```sql
-- 账号表
CREATE TABLE accounts (
  id TEXT PRIMARY KEY,
  platform TEXT NOT NULL,
  username TEXT NOT NULL,
  profile_path TEXT NOT NULL,
  proxy TEXT,
  fingerprint_preset TEXT,
  status TEXT NOT NULL,
  last_active TIMESTAMP,
  health_score INTEGER DEFAULT 100,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 任务表
CREATE TABLE tasks (
  id TEXT PRIMARY KEY,
  type TEXT NOT NULL,
  account_id TEXT NOT NULL,
  content_id TEXT,
  status TEXT NOT NULL,
  scheduled_at TIMESTAMP,
  executed_at TIMESTAMP,
  retry_count INTEGER DEFAULT 0,
  priority INTEGER DEFAULT 0,
  metadata TEXT,
  FOREIGN KEY (account_id) REFERENCES accounts(id)
);

-- 内容表
CREATE TABLE contents (
  id TEXT PRIMARY KEY,
  type TEXT NOT NULL,
  platform TEXT NOT NULL,
  title TEXT,
  body TEXT,
  media_paths TEXT,
  tags TEXT,
  source TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  used_count INTEGER DEFAULT 0
);

-- 互动记录
CREATE TABLE interactions (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL,
  target_type TEXT NOT NULL,
  target_id TEXT NOT NULL,
  action_type TEXT NOT NULL,
  status TEXT NOT NULL,
  executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (account_id) REFERENCES accounts(id)
);

-- 日志表
CREATE TABLE logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  level TEXT NOT NULL,
  module TEXT NOT NULL,
  message TEXT NOT NULL,
  account_id TEXT,
  task_id TEXT,
  timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 8.2 配置文件（YAML）

**主配置 config.yaml：**

```yaml
# 系统配置
system:
  max_concurrent_tasks: 3
  task_timeout: 300  # 秒
  log_level: info
  data_dir: "./data"

# 浏览器配置
browser:
  headless: true
  chrome_path: null  # null = 自动检测
  user_data_dir: "./profiles"
  window_size: [1920, 1080]

# Stealth 配置
stealth:
  enabled: true
  randomize_fingerprint: true
  block_webrtc: true
  timezone: "Asia/Shanghai"

# 调度配置
scheduler:
  check_interval: 60  # 秒
  retry_max: 3
  retry_backoff: [60, 300, 900]  # 秒

# 互动限制（防止触发平台限流）
rate_limits:
  xiaohongshu:
    posts_per_day: 5
    likes_per_hour: 20
    comments_per_hour: 10
    follows_per_day: 30
  twitter:
    posts_per_day: 10
    likes_per_hour: 50
    comments_per_hour: 20
    follows_per_day: 50
  instagram:
    posts_per_day: 3
    likes_per_hour: 30
    comments_per_hour: 15
    follows_per_day: 40
```

## 9. API 设计

### 9.1 REST API 端点

**账号管理：**
```
POST   /api/accounts          # 添加账号
GET    /api/accounts          # 获取账号列表
GET    /api/accounts/:id      # 获取账号详情
PUT    /api/accounts/:id      # 更新账号
DELETE /api/accounts/:id      # 删除账号
POST   /api/accounts/:id/test # 测试账号连接
```

**任务管理：**
```
POST   /api/tasks             # 创建任务
GET    /api/tasks             # 获取任务列表
GET    /api/tasks/:id         # 获取任务详情
PUT    /api/tasks/:id         # 更新任务
DELETE /api/tasks/:id         # 取消任务
POST   /api/tasks/:id/retry   # 重试任务
```

**内容管理：**
```
POST   /api/contents          # 添加内容
GET    /api/contents          # 获取内容库
GET    /api/contents/:id      # 获取内容详情
PUT    /api/contents/:id      # 更新内容
DELETE /api/contents/:id      # 删除内容
POST   /api/contents/upload   # 上传媒体文件
```

**统计与日志：**
```
GET    /api/stats             # 获取统计数据
GET    /api/stats/accounts/:id # 获取账号统计
GET    /api/logs              # 获取日志（分页）
```

### 9.2 WebSocket 实时通信

**连接端点：** `ws://localhost:8080/ws`

**消息类型：**
```json
{
  "type": "task_update",
  "data": {
    "task_id": "task_123",
    "status": "running",
    "progress": 50
  }
}

{
  "type": "log",
  "data": {
    "level": "info",
    "message": "账号 account_001 登录成功",
    "timestamp": "2026-03-17T10:30:00Z"
  }
}

{
  "type": "account_status",
  "data": {
    "account_id": "account_001",
    "status": "active",
    "health_score": 95
  }
}
```

## 10. 错误处理与安全机制

### 10.1 错误处理策略

**分层错误处理：**
- 网络错误（超时、连接失败）→ 自动重试
- 平台限流（429错误）→ 延长冷却期
- 账号异常（需要验证码）→ 暂停任务，通知用户
- 内容违规（被删除/限流）→ 记录日志，标记内容
- 浏览器崩溃 → 重启浏览器实例

**降级策略：**
- AI生成失败 → 使用模板内容
- 代理失败 → 切换备用代理或直连
- 数据库锁定 → 队列缓冲，延迟写入

### 10.2 安全与隐私

**敏感数据保护：**
- 账号密码加密存储（AES-256）
- Cookie加密存储
- 配置文件权限控制（600）
- 日志脱敏（隐藏密码、token）

**防关联措施：**
- 每个账号独立浏览器配置文件
- 不同指纹预设
- 代理IP隔离（可选）
- 操作时间随机化

### 10.3 监控与告警

**健康检查：**
- 账号登录状态检测
- 任务执行成功率
- 平台限流检测
- 系统资源监控（CPU、内存）

**告警机制：**
- 账号异常（需要人工介入）
- 任务连续失败
- 系统资源不足

## 11. 平台适配层

### 11.1 支持的平台

**国内平台（优先级）：**
1. 小红书（高）
2. 抖音（中）
3. 微博（中）
4. B站（低）

**国际平台（优先级）：**
1. Twitter/X（高）
2. Instagram（中）
3. Facebook（中）
4. LinkedIn（低）

### 11.2 平台适配器接口

```rust
#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// 登录账号
    async fn login(&self, account: &Account) -> Result<()>;

    /// 检查登录状态
    async fn check_login_status(&self) -> Result<bool>;

    /// 发布内容
    async fn post(&self, content: &Content) -> Result<String>;

    /// 点赞
    async fn like(&self, target_id: &str) -> Result<()>;

    /// 评论
    async fn comment(&self, target_id: &str, text: &str) -> Result<()>;

    /// 关注用户
    async fn follow(&self, user_id: &str) -> Result<()>;

    /// 获取动态流
    async fn get_feed(&self, limit: usize) -> Result<Vec<Post>>;

    /// 获取平台名称
    fn platform_name(&self) -> &str;
}
```

每个平台实现自己的适配器，处理平台特定的DOM选择器、API调用、反爬策略等。

## 12. CLI 与 Web 界面

### 12.1 CLI 命令设计

```bash
# 账号管理
social-auto account add --platform xiaohongshu --username user123
social-auto account list
social-auto account status account_001
social-auto account remove account_001

# 任务管理
social-auto task create --type post --account account_001 --content content_001
social-auto task create --type like --account account_001 --target-url "https://..."
social-auto task list --status pending
social-auto task cancel task_123

# 内容管理
social-auto content add --file ./post.txt --platform xiaohongshu
social-auto content list
social-auto content remove content_001

# 服务控制
social-auto start  # 启动调度器
social-auto stop
social-auto status
social-auto logs --follow

# Web界面
social-auto web --port 8080
```

### 12.2 Web 界面功能

**仪表盘页面：**
- 账号状态概览（在线/离线/异常）
- 今日任务统计（待执行/执行中/已完成/失败）
- 最近执行日志（实时更新）
- 系统资源使用（CPU、内存）

**账号管理页面：**
- 账号列表（状态、健康度、最后活跃时间）
- 添加账号表单（平台选择、账号信息、代理配置）
- 编辑/删除账号
- 手动登录/测试连接按钮

**任务管理页面：**
- 任务列表（分类：待执行、执行中、已完成、失败）
- 创建任务表单（任务类型、账号选择、内容选择、调度时间）
- 任务详情弹窗（执行日志、重试历史）
- 批量操作（取消、重试）

**内容库页面：**
- 内容列表（分类、标签、使用次数）
- 上传内容（文本编辑器、图片/视频上传）
- 内容预览和编辑
- 内容模板管理

**设置页面：**
- 系统配置（并发数、超时时间、日志级别）
- 平台限流设置（每个平台的操作频率限制）
- Stealth配置（指纹伪装开关、代理设置）
- 数据导入/导出

## 13. 项目结构

```
social-auto/
├── Cargo.toml
├── config.yaml                 # 主配置文件
├── src/
│   ├── main.rs                # 入口
│   ├── cli/                   # CLI模块
│   │   ├── mod.rs
│   │   ├── commands.rs
│   │   └── args.rs
│   ├── web/                   # Web服务
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   ├── routes/
│   │   └── websocket.rs
│   ├── core/                  # 核心业务逻辑
│   │   ├── mod.rs
│   │   ├── scheduler.rs       # 任务调度器
│   │   ├── account_manager.rs # 账号管理器
│   │   └── content_manager.rs # 内容管理器
│   ├── browser/               # 浏览器自动化
│   │   ├── mod.rs
│   │   ├── engine.rs          # Chromey封装
│   │   ├── stealth.rs         # Stealth patch
│   │   └── behavior.rs        # 行为模拟
│   ├── platforms/             # 平台适配器
│   │   ├── mod.rs
│   │   ├── trait.rs           # PlatformAdapter trait
│   │   ├── xiaohongshu.rs
│   │   ├── twitter.rs
│   │   └── ...
│   ├── models/                # 数据模型
│   │   ├── mod.rs
│   │   ├── account.rs
│   │   ├── task.rs
│   │   └── content.rs
│   ├── db/                    # 数据库
│   │   ├── mod.rs
│   │   ├── schema.rs
│   │   └── operations.rs
│   ├── config/                # 配置管理
│   │   ├── mod.rs
│   │   └── settings.rs
│   └── utils/                 # 工具函数
│       ├── mod.rs
│       ├── crypto.rs          # 加密
│       └── logger.rs          # 日志
├── web-ui/                    # Web前端（可选）
│   ├── index.html
│   ├── app.js
│   └── styles.css
├── data/                      # 数据目录
│   ├── database.db
│   └── content/
├── profiles/                  # 浏览器配置文件
│   ├── account_001/
│   └── account_002/
└── docs/
    └── superpowers/
        └── specs/
            └── 2026-03-17-social-media-automation-design.md
```

## 14. 开发路线图

### Phase 1: 基础框架（2-3周）
- 项目脚手架搭建
- 数据库设计和初始化
- 配置管理系统
- 基础CLI命令

### Phase 2: 浏览器引擎（3-4周）
- Chromey集成
- Stealth patch实现
- 行为模拟模块
- 会话管理

### Phase 3: 平台适配器（4-6周）
- 小红书适配器（优先）
- Twitter适配器（优先）
- 其他平台适配器
- 平台测试和调优

### Phase 4: 核心业务（3-4周）
- 任务调度器
- 账号管理器
- 内容管理器
- 互动策略引擎

### Phase 5: Web界面（2-3周）
- REST API实现
- WebSocket实时通信
- Web前端开发
- 界面集成测试

### Phase 6: 测试与优化（2-3周）
- 单元测试
- 集成测试
- 性能优化
- 安全加固

## 15. 风险与挑战

### 15.1 技术风险

- **平台反爬升级**：社交平台持续升级反爬策略，需要持续维护stealth patch
- **浏览器兼容性**：不同Chrome版本可能导致CDP行为差异
- **并发控制**：多账号并发操作可能导致资源竞争

### 15.2 业务风险

- **账号封禁**：即使有反检测措施，仍存在账号被封风险
- **平台政策变化**：平台ToS变更可能影响自动化合规性
- **内容审核**：AI生成内容可能触发平台审核机制

### 15.3 缓解措施

- 保守的操作频率限制
- 完善的日志和监控系统
- 账号健康度评分机制
- 人工审核关键内容
- 定期更新stealth patch

## 16. 成功标准

- 支持至少2个国内平台和2个国际平台
- 单账号每日可稳定发布5+条内容
- 互动任务成功率 > 90%
- 账号存活率 > 80%（30天周期）
- Web界面响应时间 < 200ms
- 系统内存占用 < 500MB（单账号）

## 17. 后续扩展方向

- 数据分析和报表功能
- 内容效果追踪（点赞数、评论数、转发数）
- AI内容生成优化（更智能的评论、更贴合平台风格的内容）
- 移动端管理应用
- 分布式部署支持
- 更多平台支持（TikTok、YouTube等）
