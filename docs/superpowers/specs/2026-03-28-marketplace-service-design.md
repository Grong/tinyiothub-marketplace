# Marketplace Service — 产品设计与实现方案

> **项目**: TinyIoTHub 云端市场服务
> **日期**: 2026-03-28
> **状态**: 设计完成，待实现

---

## 1. 背景与目标

### 1.1 项目背景

TinyIoTHub 目前已具备成熟的市场模块，支持模板和驱动的本地管理（`source_type = "local"`）和 GitHub 托管（`source_type = "github"`）。为进一步支持多网关共享、独立迭代和 SaaS 化，需要将市场模块独立为单独的服务。

### 1.2 核心目标

- **多网关共享** — 多个边缘网关设备从同一个市场源获取模板/驱动
- **独立迭代** — 市场功能（发布、审核、统计）与核心网关解耦
- **SaaS 化** — 对外提供 Marketplace API 服务

### 1.3 技术栈

| 组件 | 选择 |
|------|------|
| Rust | Nightly 2025-11-30 |
| Web 框架 | Axum 0.8 |
| 异步运行时 | Tokio 1.x |
| 数据库 | PostgreSQL (生产) / SQLite (开发) |
| 缓存 | Sled (嵌入式 KV) |
| 前端框架 | Lit 3.3.2 (Web Components) |
| 构建工具 | Vite 7.3.1 |
| 状态管理 | @lit-labs/signals |

### 1.4 部署架构

```
┌─────────────────────────────────────────────────────────────────────┐
│                 marketplace.tinyiothub.com (独立服务)                 │
│                                                                      │
│  ┌──────────────┐         ┌──────────────┐         ┌────────────┐   │
│  │   Lit 3.3    │         │   Axum 0.8   │         │  Sled 3.0  │   │
│  │  Frontend    │  ←──→   │  API Server  │  ←──→   │   Cache    │   │
│  │ (Web Comp.)  │  JSON   │   (Port 3003) │         │            │   │
│  └──────────────┘         └──────┬───────┘         └────────────┘   │
│                                   │                                │
│                          ┌────────┴────────┐                       │
│                          │  PostgreSQL     │                       │
│                          │  (元数据存储)   │                       │
│                          └─────────────────┘                       │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  GitHub Webhook → Webhook Handler → Sled → PostgreSQL          │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                    (JSON 索引 + 资源文件)
                                    ↓
┌─────────────────────────────────────────────────────────────────────┐
│                     TinyIoTHub 网关实例                              │
│                                                                      │
│  配置: source_type = "api", api_url = "https://marketplace.tiny... │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. 仓库结构

```
tinyiothub-marketplace/
│
├── api/                         # Axum API 服务
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs
│       ├── api/                 # HTTP handlers, routes
│       │   ├── mod.rs
│       │   ├── v1/
│       │   │   ├── mod.rs
│       │   │   ├── templates.rs
│       │   │   ├── drivers.rs
│       │   │   └── webhook.rs
│       │   └── health.rs
│       ├── domain/              # Business logic
│       │   ├── mod.rs
│       │   ├── template.rs
│       │   └── driver.rs
│       ├── infrastructure/       # DB, cache, GitHub client
│       │   ├── mod.rs
│       │   ├── database.rs
│       │   ├── cache.rs
│       │   └── github.rs
│       └── dto/                 # Request/Response types
│           ├── mod.rs
│           ├── request.rs
│           └── response.rs
│
├── web/                         # Lit 前端
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   └── src/
│       ├── main.ts
│       ├── components/
│       │   ├── marketplace-app.ts
│       │   ├── marketplace-header.ts
│       │   ├── template-list.ts
│       │   ├── template-card.ts
│       │   ├── driver-list.ts
│       │   ├── driver-card.ts
│       │   ├── download-button.ts
│       │   ├── category-filter.ts
│       │   ├── protocol-filter.ts
│       │   ├── search-box.ts
│       │   └── sort-dropdown.ts
│       └── styles/
│           └── global.css
│
├── docs/                        # 设计文档
│   └── superpowers/
│       └── specs/
│           └── 2026-03-28-marketplace-service-design.md
│
├── docker-compose.yml            # 本地开发
├── Dockerfile                   # 生产构建
└── README.md
```

---

## 3. API 设计

### 3.1 端点清单

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/v1/templates` | 模板列表（分页、筛选、搜索） |
| GET | `/v1/templates/:id` | 模板详情 |
| GET | `/v1/templates/:id/download` | 代理下载模板文件 |
| GET | `/v1/drivers` | 驱动列表（分页、筛选、搜索） |
| GET | `/v1/drivers/:id` | 驱动详情 |
| GET | `/v1/drivers/:id/download` | 代理下载驱动文件 |
| POST | `/v1/webhook/github` | GitHub Webhook 接收器 |
| GET | `/health` | 健康检查 |

### 3.2 响应格式

```json
{
  "code": 0,
  "msg": "",
  "result": { ... }
}
```

与 TinyIoTHub API 保持完全一致。

### 3.3 列表 API 查询参数

```
GET /v1/templates?page=1&page_size=20&category=sensor&protocol=modbus&search=温度
```

| 参数 | 类型 | 说明 |
|------|------|------|
| `page` | int | 页码，默认 1 |
| `page_size` | int | 每页条数，默认 20，最大 100 |
| `category` | string | 分类筛选 |
| `protocol` | string | 协议筛选 |
| `search` | string | 关键词搜索（名称/描述） |

### 3.4 下载统计

每次 `/download` 请求：
1. Sled 缓存中原子递增 `download_count:{resource_id}`
2. 异步写入 PostgreSQL `download_logs` 表

---

## 4. 数据模型

### 4.1 PostgreSQL 表结构

```sql
-- 模板表
CREATE TABLE templates (
    id              VARCHAR(64) PRIMARY KEY,
    name            VARCHAR(128) NOT NULL,
    version         VARCHAR(16) NOT NULL,
    category        VARCHAR(32) NOT NULL,
    protocol        VARCHAR(32) NOT NULL,
    manufacturer    VARCHAR(64),
    description     TEXT,
    tags            TEXT[],
    author_name     VARCHAR(64) NOT NULL,
    author_email    VARCHAR(128),
    icon            VARCHAR(256),
    downloads       BIGINT DEFAULT 0,
    rating          DECIMAL(3,2) DEFAULT 0,
    reviews         INT DEFAULT 0,
    license         VARCHAR(32) NOT NULL,
    file_url        VARCHAR(512) NOT NULL,
    checksum        VARCHAR(128) NOT NULL,
    size            BIGINT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL,
    updated_at      TIMESTAMPTZ NOT NULL
);

-- 驱动表
CREATE TABLE drivers (
    id              VARCHAR(64) PRIMARY KEY,
    name            VARCHAR(128) NOT NULL,
    version         VARCHAR(16) NOT NULL,
    protocol        VARCHAR(32) NOT NULL,
    description     TEXT,
    tags            TEXT[],
    author_name     VARCHAR(64) NOT NULL,
    author_email    VARCHAR(128),
    icon            VARCHAR(256),
    downloads       BIGINT DEFAULT 0,
    rating          DECIMAL(3,2) DEFAULT 0,
    reviews         INT DEFAULT 0,
    license         VARCHAR(32) NOT NULL,
    homepage        VARCHAR(256),
    documentation   VARCHAR(256),
    platforms       JSONB NOT NULL,
    requirements    JSONB,
    created_at      TIMESTAMPTZ NOT NULL,
    updated_at      TIMESTAMPTZ NOT NULL
);

-- 下载日志表
CREATE TABLE download_logs (
    id              BIGSERIAL PRIMARY KEY,
    resource_type   VARCHAR(16) NOT NULL,
    resource_id     VARCHAR(64) NOT NULL,
    downloaded_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_agent      VARCHAR(256),
    ip_address      VARCHAR(45)
);

-- 索引
CREATE INDEX idx_templates_category ON templates(category);
CREATE INDEX idx_templates_protocol ON templates(protocol);
CREATE INDEX idx_drivers_protocol ON drivers(protocol);
CREATE INDEX idx_download_logs_resource ON download_logs(resource_type, resource_id);
```

### 4.2 Sled 缓存设计

| Key 模式 | Value | 说明 |
|----------|-------|------|
| `templates_index` | JSON bytes | 完整模板列表缓存 |
| `drivers_index` | JSON bytes | 完整驱动列表缓存 |
| `download_count:{id}` | i64 | 下载计数器（原子递增） |
| `last_sync` | i64 (timestamp) | 最后同步时间 |

---

## 5. GitHub Webhook 同步

### 5.1 Webhook 配置

```
GitHub Webhook URL: https://marketplace.tinyiothub.com/v1/webhook/github
Content-Type: application/json
Secret: <配置的 HMAC 密钥>
Events: push
```

### 5.2 同步流程

```
GitHub push → POST /v1/webhook/github
                         │
                         ▼
              ┌──────────────────────────┐
              │ 1. 验证 HMAC signature   │
              │    (X-Hub-Signature-256) │
              └────────────┬─────────────┘
                           │
                           ▼
              ┌──────────────────────────┐
              │ 2. 解析 payload          │
              │   提取 changed files     │
              └────────────┬─────────────┘
                           │
                           ▼
              ┌──────────────────────────┐
              │ 3. 判断变更类型           │
              │   templates/ → 同步模板   │
              │   drivers/ → 同步驱动    │
              │   index.json → 全量同步   │
              └────────────┬─────────────┘
                           │
                           ▼
              ┌──────────────────────────┐
              │ 4. Sled 缓存更新         │
              │   (写入 + 设置 last_sync) │
              └────────────┬─────────────┘
                           │
                           ▼
              ┌──────────────────────────┐
              │ 5. PostgreSQL 异步写入   │
              │   (FROM Sled snapshot)   │
              └──────────────────────────┘
```

### 5.3 容错设计

- Webhook 处理超时：30 秒
- 失败重试：返回 500，GitHub 会自动重试
- Sled 写入优先，PostgreSQL 失败不影响缓存更新
- 全量 index.json 变更时，先更新 Sled，再清理旧数据

---

## 6. Lit 前端组件

### 6.1 组件清单

| 组件 | 描述 |
|------|------|
| `<marketplace-app>` | 根组件，布局 + 路由 |
| `<marketplace-header>` | 顶部导航栏（无边框设计） |
| `<template-list>` | 模板列表（含搜索/筛选） |
| `<template-card>` | 模板卡片 |
| `<driver-list>` | 驱动列表（含搜索/筛选） |
| `<driver-card>` | 驱动卡片 |
| `<download-button>` | 下载按钮（含统计） |
| `<category-filter>` | 分类筛选器 |
| `<protocol-filter>` | 协议筛选器 |
| `<search-box>` | 搜索框 |
| `<sort-dropdown>` | 排序下拉 |

### 6.2 无边框设计

所有组件作为 Web Components 嵌入到父页面时没有浏览器边框，纯内容展示。

### 6.3 状态管理

使用 `@lit-labs/signals`：

```typescript
const marketplaceState = {
  activeTab: signal<'templates' | 'drivers'>('templates'),
  templates: signal<Template[]>([]),
  drivers: signal<Driver[]>([]),
  loading: signal(false),
  searchQuery: signal(''),
  categoryFilter: signal<string | null>(null),
  protocolFilter: signal<string | null>(null),
}
```

---

## 7. 部署

### 7.1 开发环境（Docker Compose）

```yaml
# docker-compose.yml
services:
  marketplace-api:
    build:
      context: ./api
      dockerfile: Dockerfile
    ports:
      - "3003:3003"
    environment:
      - DATABASE_URL=postgres://user:pass@db:5432/marketplace
      - SLED_PATH=/data/marketplace.sled
      - GITHUB_WEBHOOK_SECRET=${GITHUB_WEBHOOK_SECRET}
    volumes:
      - sled_data:/data
    depends_on:
      - db

  db:
    image: postgres:16-alpine
    environment:
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
      - POSTGRES_DB=marketplace
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  sled_data:
  postgres_data:
```

### 7.2 生产环境

- 二进制或 Docker 单独运行 `marketplace-api`
- 服务端口：3003
- PostgreSQL 可共用服务器已有实例或独立容器
- 前端静态资源由 Axum serve_static 提供或 Nginx 反向代理

### 7.3 配置项

| 环境变量 | 说明 |
|----------|------|
| `DATABASE_URL` | PostgreSQL 连接串 |
| `SLED_PATH` | Sled 缓存路径 |
| `GITHUB_WEBHOOK_SECRET` | GitHub HMAC 密钥 |
| `RUST_LOG` | 日志级别 |
| `PORT` | 服务端口（默认 3003） |

---

## 8. TinyIoTHub 侧变更

### 8.1 配置说明

现有配置已支持云端市场模式，只需将 `api_url` 指向新服务：

```toml
# 本地内置（默认）
[marketplace]
enabled = true
source_type = "local"
local_path = "marketplace"

# GitHub 托管
[marketplace]
enabled = true
source_type = "github"
github_repo = "tinyiothub/marketplace"
github_branch = "main"

# 云端市场
[marketplace]
enabled = true
source_type = "api"
api_url = "https://marketplace.tinyiothub.com/v1"
```

### 8.2 调用逻辑

```
source_type = "api":
  TinyIoTHub → Marketplace API 获取元数据
  TinyIoTHub → Marketplace API /download 代理下载
  (本地内置 JSON 作为 fallback)
```

---

## 9. 实施计划

### Phase 1（当前）

- [ ] 项目骨架搭建（API + 前端）
- [ ] PostgreSQL 表结构 + Sled 缓存
- [ ] `/v1/templates` 和 `/v1/drivers` 端点
- [ ] GitHub Webhook 同步
- [ ] Lit 无边框前端
- [ ] Docker Compose 开发环境
- [ ] 与 TinyIoTHub 集成测试

### Phase 2（未来）

- [ ] 下载统计展示
- [ ] OAuth 认证（管理后台）
- [ ] 开发者上传/发布流程
- [ ] 评分/评论系统

---

## 10. 技术决策记录

| 决策 | 选择 | 理由 |
|------|------|------|
| 独立仓库 | `tinyiothub/marketplace-service` | 独立部署、迭代、贡献 |
| API 版本化 | `/v1/` 前缀 | 标准化，支持未来兼容 |
| 下载方式 | 代理下载 | 可统计下载量 |
| 数据源 | 静态 JSON + GitHub Webhook | 与现有 `marketplace/` 目录一致 |
| 缓存策略 | Sled + PostgreSQL | Sled 加速查询，PG 持久化 |
| 前端集成 | 独立子域名 | 与 Next.js 完全分离 |
