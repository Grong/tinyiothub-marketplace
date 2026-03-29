# TinyIoT Hub Marketplace

IoT 设备模板和驱动市场 API 服务。

## 功能

- **模板市场**: 浏览、搜索、下载 IoT 设备模板
- **驱动市场**: 浏览、搜索、下载 IoT 驱动
- **自动同步**: GitHub webhook 推送时自动同步更新
- **定时同步**: 每小时从配置的 GitHub 仓库拉取最新数据

## API 端点

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/health` | 健康检查 |
| GET | `/v1/templates` | 模板列表（支持分页、搜索、分类筛选） |
| GET | `/v1/templates/{id}` | 获取模板详情 |
| GET | `/v1/templates/{id}/download` | 下载模板 |
| GET | `/v1/drivers` | 驱动列表（支持分页、搜索、分类筛选） |
| GET | `/v1/drivers/{id}` | 获取驱动详情 |
| GET | `/v1/drivers/{id}/download` | 下载驱动 |
| POST | `/v1/webhook/github` | GitHub Webhook（接收 push 事件） |

### 查询参数

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `page` | int | 1 | 页码 |
| `per_page` | int | 20 | 每页数量（最大 100） |
| `search` | string | - | 关键词搜索 |
| `category` | string | - | 分类筛选 |
| `protocol` | string | - | 协议筛选 |

## 快速开始

### Docker Compose（推荐）

```bash
# 克隆仓库
git clone https://github.com/Grong/tinyiothub-marketplace.git
cd tinyiothub-marketplace

# 配置环境变量
cp .env.example .env
# 编辑 .env 填入你的 GitHub Token

# 启动服务
docker-compose up -d
```

服务将在 `http://localhost:3003` 启动。

### 本地开发

```bash
cd api

# 安装 Rust nightly
rustup default nightly

# 运行
cargo run
```

## 配置

### 环境变量

| 变量 | 默认值 | 描述 |
|------|--------|------|
| `PORT` | 3003 | 服务端口 |
| `GITHUB_TOKEN` | dummy | GitHub API Token |
| `GITHUB_WEBHOOK_SECRET` | dummy | Webhook 密钥 |
| `SLED_PATH` | /tmp/marketplace.sled | 数据库路径 |
| `REPOS_CONFIG` | config/repositories.json | 仓库配置文件 |
| `LOCAL_DATA_PATH` | - | 本地数据目录（开发用） |
| `RUST_LOG` | info | 日志级别 |

### 仓库配置

编辑 `config/repositories.json`:

```json
[
  { "type": "template", "repo": "Grong/tinyiothub-marketplace-repo", "path": "templates" },
  { "type": "driver", "repo": "Grong/tinyiothub-marketplace-repo", "path": "drivers" }
]
```

## Docker

### 构建镜像

```bash
cd api
docker build -t tinyiothub-marketplace .
```

### 运行容器

```bash
docker run -d \
  -p 3003:3003 \
  -e GITHUB_TOKEN=your_token \
  -e GITHUB_WEBHOOK_SECRET=your_secret \
  -v ./data:/data \
  -v ./config:/app/config:ro \
  tinyiothub-marketplace
```

### 多架构构建

```bash
docker buildx build --platforms linux/amd64,linux/arm64 -t tinyiothub-marketplace --push .
```

## CI/CD

GitHub Actions 自动构建 Docker 镜像并推送到 Docker Hub：

- 推送 tag 时触发构建
- 镜像标签: `0.0.1`, `0.0`, `latest`

## 项目结构

```
.
├── api/                    # Rust API 服务
│   └── src/
│       ├── api/v1/         # API 处理器
│       │   ├── templates.rs
│       │   ├── drivers.rs
│       │   └── webhook.rs
│       ├── domain/         # 领域模型
│       ├── dto/           # 数据传输对象
│       ├── infrastructure/ # 基础设施
│       │   ├── cache.rs   # Sled 缓存
│       │   └── github.rs  # GitHub 客户端
│       └── sync/          # 同步服务
├── config/                 # 配置文件
│   └── repositories.json
├── docker-compose.yml
└── Dockerfile
```

## 技术栈

- **框架**: Axum 0.8
- **运行时**: Tokio
- **数据库**: Sled（嵌入式）
- **HTTP 客户端**: Reqwest
- **认证**: HMAC-SHA256 Webhook 验证
