# TinyIoT Hub Marketplace API

API 文档。

## 基础信息

- **Base URL**: `http://localhost:3003/api`
- **响应格式**: JSON
- **认证**: 无（公开 API）

## 通用响应格式

### 成功响应

```json
{
  "code": 0,
  "msg": "",
  "result": { ... }
}
```

### 错误响应

```json
{
  "code": 40001,
  "msg": "error message",
  "result": null
}
```

### 分页响应

```json
{
  "code": 0,
  "msg": "",
  "result": {
    "items": [...],
    "total": 100,
    "page": 1,
    "per_page": 20
  }
}
```

### HTTP 状态码

| 状态码 | 含义 |
|--------|------|
| 200 | 成功 |
| 400 | 请求参数错误 |
| 401 | 未授权（Webhook 验证失败） |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

---

## 健康检查

### GET /health

检查服务健康状态和最后同步时间。

**响应示例**:

```json
{
  "code": 0,
  "msg": "",
  "result": {
    "status": "healthy",
    "last_sync": "2026-03-29T00:00:00+00:00"
  }
}
```

---

## 模板 API

### GET /api/v1/templates

获取模板列表。

**查询参数**:

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `page` | int | 1 | 页码（最小 1） |
| `per_page` | int | 20 | 每页数量（1-100） |
| `search` | string | - | 关键词搜索（name, description） |
| `category` | string | - | 按分类筛选 |
| `protocol` | string | - | 按协议筛选（modbus, mqtt 等） |

**响应示例**:

```json
{
  "code": 0,
  "msg": "",
  "result": {
    "items": [
      {
        "id": "template-001",
        "name": "温湿度传感器",
        "version": "1.0.0",
        "category": "sensor",
        "protocol": "modbus",
        "manufacturer": "TinyIoT",
        "description": "支持温湿度数据采集的设备模板",
        "tags": ["temperature", "humidity", "modbus"],
        "author_name": "tinyiothub",
        "icon": null,
        "license": "MIT",
        "downloads": 0,
        "rating": 5.0,
        "size": 102400,
        "updated_at": "2026-03-29T00:00:00+00:00"
      }
    ],
    "total": 2,
    "page": 1,
    "per_page": 20
  }
}
```

---

### GET /api/v1/templates/{id}

获取模板详情。

**路径参数**:

| 参数 | 类型 | 描述 |
|------|------|------|
| `id` | string | 模板 ID |

**响应示例**:

```json
{
  "code": 0,
  "msg": "",
  "result": {
    "id": "template-001",
    "name": "温湿度传感器",
    "version": "1.0.0",
    "category": "sensor",
    "protocol": "modbus",
    "manufacturer": "TinyIoT",
    "description": "支持温湿度数据采集的设备模板",
    "tags": ["temperature", "humidity", "modbus"],
    "author_name": "tinyiothub",
    "author_email": null,
    "icon": null,
    "license": "MIT",
    "file_url": "https://github.com/Grong/tinyiothub-marketplace-repo/raw/main/templates/template-001.zip",
    "checksum": null,
    "readme_url": null,
    "size": 102400,
    "updated_at": "2026-03-29T00:00:00+00:00"
  }
}
```

**错误响应** (404):

```json
{
  "code": 40401,
  "msg": "template not found",
  "result": null
}
```

---

### GET /api/v1/templates/{id}/download

下载模板文件。

请求后重定向到 `file_url`。

**路径参数**:

| 参数 | 类型 | 描述 |
|------|------|------|
| `id` | string | 模板 ID |

**响应**: 302 重定向到 GitHub 下载地址。

**URL 验证**:
- 必须使用 HTTPS
- 只允许来自 `github.com` 和 `raw.githubusercontent.com` 的链接

**错误响应**:

| 状态码 | code | 描述 |
|--------|------|------|
| 400 | 40001 | 无效的 URL |
| 400 | 40002 | URL 未使用 HTTPS |
| 400 | 40004 | URL 域名不在允许列表 |
| 404 | 40401 | 模板不存在 |
| 500 | 50001 | 模板缺少 file_url 字段 |

---

## 驱动 API

### GET /api/v1/drivers

获取驱动列表。

**查询参数**:

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `page` | int | 1 | 页码（最小 1） |
| `per_page` | int | 20 | 每页数量（1-100） |
| `search` | string | - | 关键词搜索 |
| `category` | string | - | 按分类筛选 |
| `protocol` | string | - | 按协议筛选 |

**响应格式** 同 `/api/v1/templates`。

---

### GET /api/v1/drivers/{id}

获取驱动详情。

**路径参数**:

| 参数 | 类型 | 描述 |
|------|------|------|
| `id` | string | 驱动 ID |

**响应示例**:

```json
{
  "code": 0,
  "msg": "",
  "result": {
    "id": "driver-001",
    "name": "Modbus RTU Driver",
    "version": "1.0.0",
    "protocol": "modbus",
    "description": "Modbus RTU 通信驱动",
    "tags": ["modbus", "rtu", "industrial"],
    "author_name": "tinyiothub",
    "author_email": null,
    "icon": null,
    "license": "MIT",
    "homepage": "https://github.com/example/driver",
    "documentation": "https://github.com/example/driver#readme",
    "platforms": ["linux/amd64", "linux/arm64"],
    "requirements": { "os": "linux", "arch": ["amd64", "arm64"] },
    "updated_at": "2026-03-29T00:00:00+00:00"
  }
}
```

---

### GET /api/v1/drivers/{id}/download

下载驱动。

优先使用 `homepage`，其次 `documentation`，最后默认到 GitHub。

**路径参数**:

| 参数 | 类型 | 描述 |
|------|------|------|
| `id` | string | 驱动 ID |

**响应**: 302 重定向到驱动下载地址。

**URL 验证** 同模板下载。

---

## Webhook API

### POST /api/v1/webhook/github

接收 GitHub push 事件，触发同步。

**请求头**:

| 头 | 描述 |
|-----|------|
| `X-Hub-Signature-256` | HMAC-SHA256 签名（`sha256=...`） |
| `X-GitHub-Event` | 事件类型（必须是 `push`） |
| `X-GitHub-Delivery` | 投递 ID（用于幂等性检查） |

**HMAC 验证**:

```python
import hmac
import hashlib

signature = hmac.new(
    secret.encode(),
    payload.encode(),
    hashlib.sha256
).hexdigest()

# 验证
expected = f"sha256={signature}"
if not hmac.compare_digest(f"sha256={signature}", header_value):
    return 401
```

**请求体** (GitHub push 事件):

```json
{
  "ref": "refs/heads/main",
  "commits": [...]
}
```

**响应**:

| 状态码 | 描述 |
|--------|------|
| 200 | 处理成功（包括忽略的非 push 事件） |
| 401 | HMAC 验证失败或未提供签名 |
| 500 | 服务器配置错误（未设置 WEBHOOK_SECRET） |

**幂等性**: 相同 `X-GitHub-Delivery` 不会重复处理。

---

## 错误码

| code | 含义 |
|------|------|
| 0 | 成功 |
| 400 | 请求参数错误 |
| 401 | 未授权 |
| 40401 | 模板不存在 |
| 40402 | 驱动不存在 |
| 40001 | 无效的 URL |
| 40002 | URL 必须使用 HTTPS |
| 40003 | URL 缺少 host |
| 40004 | URL 域名不允许 |
| 50001 | 模板缺少 file_url |
| 50002 | 服务器未配置 Webhook Secret |
| 502 | 缓存服务不可用 |

---

## 数据模型

### Template

| 字段 | 类型 | 必填 | 描述 |
|------|------|------|------|
| `id` | string | 是 | 唯一标识（字母、数字、-、_） |
| `name` | string | 是 | 名称 |
| `version` | string | 是 | 版本号（数字、.、-） |
| `category` | string | 否 | 分类 |
| `protocol` | string | 是 | 协议（modbus, mqtt 等） |
| `manufacturer` | string | 否 | 制造商 |
| `description` | string | 是 | 描述 |
| `tags` | string[] | 否 | 标签 |
| `author_name` | string | 是 | 作者名 |
| `author_email` | string | 否 | 作者邮箱 |
| `icon` | string | 否 | 图标 URL |
| `license` | string | 是 | 许可证 |
| `file_url` | string | 是 | 下载地址 |
| `checksum` | string | 否 | 文件校验码 |
| `readme_url` | string | 否 | README URL |
| `size` | int | 否 | 文件大小（字节） |
| `downloads` | int | 否 | 下载次数 |
| `rating` | float | 否 | 评分 |
| `updated_at` | string | 是 | 更新时间（ISO 8601） |

### Driver

| 字段 | 类型 | 必填 | 描述 |
|------|------|------|------|
| `id` | string | 是 | 唯一标识 |
| `name` | string | 是 | 名称 |
| `version` | string | 是 | 版本号 |
| `protocol` | string | 是 | 协议 |
| `description` | string | 是 | 描述 |
| `tags` | string[] | 否 | 标签 |
| `author_name` | string | 是 | 作者名 |
| `author_email` | string | 否 | 作者邮箱 |
| `license` | string | 是 | 许可证 |
| `homepage` | string | 否 | 主页 |
| `documentation` | string | 否 | 文档 URL |
| `platforms` | object | 否 | 支持的平台 |
| `requirements` | object | 否 | 依赖要求 |
| `updated_at` | string | 是 | 更新时间 |
