# TinyIoT Hub Marketplace API

API 文档。

## 基础信息

- **Base URL**: `http://localhost:3003`
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
  "code": 400,
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
| 404 | 资源不存在 |
| 502 | 缓存服务不可用 |
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
    "status": "ok",
    "last_sync": "2026-03-29T10:43:06+00:00"
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
| `search` | string | - | 关键词搜索（name, display_name） |
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
        "name": "humidity_sensor",
        "display_name": { "zh": "湿度传感器", "en": "Humidity Sensor" },
        "description": { "zh": "标准湿度传感器设备模板", "en": "Standard humidity sensor device template" },
        "version": "1.0.0",
        "author": "TinyIoT",
        "category": "sensors",
        "manufacturer": null,
        "device_type": "sensor",
        "protocol_type": "modbus",
        "driver_name": "modbus_rtu",
        "tags": ["sensor", "humidity", "monitoring"]
      }
    ],
    "total": 20,
    "page": 1,
    "per_page": 20
  }
}
```

---

### GET /api/v1/templates/{name}

获取模板详情。

**路径参数**:

| 参数 | 类型 | 描述 |
|------|------|------|
| `name` | string | 模板名称（唯一标识） |

**响应示例**:

```json
{
  "code": 0,
  "msg": "",
  "result": {
    "name": "humidity_sensor",
    "display_name": { "zh": "湿度传感器", "en": "Humidity Sensor" },
    "description": { "zh": "标准湿度传感器设备模板", "en": "Standard humidity sensor device template" },
    "version": "1.0.0",
    "author": "TinyIoT",
    "category": "sensors",
    "manufacturer": null,
    "device_type": "sensor",
    "protocol_type": "modbus",
    "driver_name": "modbus_rtu",
    "tags": ["sensor", "humidity", "monitoring"],
    "device_info": {
      "default_name_pattern": "humidity_sensor_{index}",
      "default_display_name_pattern": { "zh": "湿度传感器 {index}", "en": "Humidity Sensor {index}" },
      "required_fields": ["name", "address"]
    },
    "properties": [
      {
        "name": "humidity",
        "display_name": { "zh": "湿度", "en": "Humidity" },
        "description": { "zh": "当前环境湿度", "en": "Current ambient humidity" },
        "data_type": "number",
        "unit": "%RH",
        "min_value": 0.0,
        "max_value": 100.0,
        "default_value": "50.0",
        "is_read_only": true,
        "is_required": true
      }
    ],
    "commands": [
      {
        "name": "read_humidity",
        "display_name": { "zh": "读取湿度", "en": "Read Humidity" },
        "description": { "zh": "读取当前湿度值", "en": "Read current humidity value" },
        "parameters": "{}",
        "parameter_schema": null,
        "is_required": true
      }
    ]
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

## 驱动 API

### GET /api/v1/drivers

获取驱动列表。

**查询参数**:

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `page` | int | 1 | 页码 |
| `per_page` | int | 20 | 每页数量（1-100） |
| `search` | string | - | 关键词搜索 |
| `protocol` | string | - | 按协议筛选 |

**响应格式**: 同 `/api/v1/templates`。

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
    "documentation": null,
    "platforms": null,
    "requirements": null,
    "updated_at": "2026-03-29T00:00:00+00:00"
  }
}
```

---

## 错误码

| code | 含义 |
|------|------|
| 0 | 成功 |
| 400 | 请求参数错误 |
| 40401 | 模板不存在 |
| 40402 | 驱动不存在 |
| 500 | 服务器内部错误 |
| 502 | 缓存服务不可用 |

---

## 数据模型

### Template

| 字段 | 类型 | 必填 | 描述 |
|------|------|------|------|
| `name` | string | 是 | 模板唯一标识（英文） |
| `display_name` | object | 是 | 多语言显示名称 { zh, en } |
| `description` | object | 是 | 多语言描述 { zh, en } |
| `version` | string | 是 | 版本号 |
| `author` | string | 是 | 作者 |
| `category` | string | 是 | 分类（sensors, actuators, meters 等） |
| `manufacturer` | string | 否 | 制造商 |
| `device_type` | string | 是 | 设备类型 |
| `protocol_type` | string | 是 | 协议类型 |
| `driver_name` | string | 是 | 关联的驱动名称 |
| `tags` | string[] | 否 | 标签 |
| `device_info` | object | 否 | 设备实例默认配置 |
| `properties` | object[] | 否 | 属性定义列表 |
| `commands` | object[] | 否 | 命令定义列表 |

### Property

| 字段 | 类型 | 描述 |
|------|------|------|
| `name` | string | 属性唯一标识 |
| `display_name` | object | 多语言显示名称 |
| `description` | object | 多语言描述 |
| `data_type` | string | 数据类型（number, string, boolean） |
| `unit` | string | 单位 |
| `min_value` | number | 最小值 |
| `max_value` | number | 最大值 |
| `default_value` | string | 默认值 |
| `is_read_only` | boolean | 是否只读 |
| `is_required` | boolean | 是否必填 |

### Command

| 字段 | 类型 | 描述 |
|------|------|------|
| `name` | string | 命令唯一标识 |
| `display_name` | object | 多语言显示名称 |
| `description` | object | 多语言描述 |
| `parameters` | string | 默认参数字符串 |
| `parameter_schema` | string | JSON Schema 格式的参数定义 |
| `is_required` | boolean | 是否必填 |

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
| `updated_at` | string | 是 | 更新时间（ISO 8601） |
