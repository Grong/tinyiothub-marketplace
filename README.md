# TinyIoT Hub Marketplace

IoT 设备模板和驱动市场 API 服务。

## 功能

- **模板市场**: 浏览、搜索、下载 IoT 设备模板
- **驱动市场**: 浏览、搜索、下载 IoT 驱动
- **本地数据**: 数据存储在本地文件系统，部署简单

## API 端点

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/health` | 健康检查 |
| GET | `/v1/templates` | 模板列表（支持分页、搜索、分类筛选） |
| GET | `/v1/templates/{name}` | 获取模板详情 |
| GET | `/v1/drivers` | 驱动列表（支持分页、搜索、协议筛选） |
| GET | `/v1/drivers/{id}` | 获取驱动详情 |

### 查询参数

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `page` | int | 1 | 页码 |
| `per_page` | int | 20 | 每页数量（最大 100） |
| `search` | string | - | 关键词搜索 |
| `category` | string | - | 分类筛选（仅模板） |
| `protocol` | string | - | 协议筛选 |

## 快速开始

### Docker Compose（推荐）

```bash
# 克隆仓库
git clone https://github.com/Grong/tinyiothub-marketplace.git
cd tinyiothub-marketplace

# 启动服务
docker-compose up -d
```

服务将在 `http://localhost:3003` 启动。

### 本地开发

```bash
cd api

# 安装 Rust
rustup default stable

# 运行
cargo run
```

## 数据管理

模板和驱动数据存储在本地 JSON 文件中：

```
data/
├── templates/
│   ├── humidity_sensor.json    # 一个模板一个文件
│   └── temperature_sensor.json
└── drivers/
    └── modbus_rtu.json         # 一个驱动一个文件
```

每个 `.json` 文件包含一个模板或驱动的完整定义。

## 数据规范

### 模板规范 (Template)

模板定义设备类型，包含属性和命令配置。

```json
{
  "name": "humidity_sensor",
  "display_name": {
    "zh": "湿度传感器",
    "en": "Humidity Sensor"
  },
  "description": {
    "zh": "标准湿度传感器设备模板",
    "en": "Standard humidity sensor device template"
  },
  "version": "1.0.0",
  "author": "System",
  "category": "sensors",
  "manufacturer": null,
  "device_type": "sensor",
  "protocol_type": "modbus",
  "driver_name": "modbus_rtu",
  "tags": ["sensor", "humidity", "monitoring"],
  "device_info": {
    "default_name_pattern": "humidity_sensor_{index}",
    "default_display_name_pattern": {
      "zh": "湿度传感器 {index}",
      "en": "Humidity Sensor {index}"
    },
    "required_fields": ["name", "address"]
  },
  "properties": [...],
  "commands": [...]
}
```

#### 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 模板唯一标识（英文） |
| `display_name` | object | 是 | 多语言显示名称 |
| `description` | object | 是 | 多语言描述 |
| `version` | string | 是 | 版本号 |
| `author` | string | 是 | 作者 |
| `category` | string | 是 | 分类（如 sensors, actuators） |
| `manufacturer` | string | 否 | 制造商 |
| `device_type` | string | 是 | 设备类型 |
| `protocol_type` | string | 是 | 协议类型（如 modbus, bacnet） |
| `driver_name` | string | 是 | 关联的驱动名称 |
| `tags` | array | 否 | 标签 |
| `device_info` | object | 否 | 设备实例默认配置 |
| `properties` | array | 否 | 属性定义列表 |
| `commands` | array | 否 | 命令定义列表 |

#### 属性 (Property) 结构

```json
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
```

| 字段 | 类型 | 说明 |
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

#### 命令 (Command) 结构

```json
{
  "name": "read_humidity",
  "display_name": { "zh": "读取湿度", "en": "Read Humidity" },
  "description": { "zh": "读取当前湿度值", "en": "Read current humidity value" },
  "parameters": "{}",
  "parameter_schema": "{\"type\": \"object\", \"properties\": {...}}",
  "is_required": false
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 命令唯一标识 |
| `display_name` | object | 多语言显示名称 |
| `description` | object | 多语言描述 |
| `parameters` | string | 默认参数字符串 |
| `parameter_schema` | string | JSON Schema 格式的参数定义 |
| `is_required` | boolean | 是否必填 |

### 驱动规范 (Driver)

驱动规范设计中，后续补充。

## 配置

### 环境变量

| 变量 | 默认值 | 描述 |
|------|--------|------|
| `PORT` | 3003 | 服务端口 |
| `SLED_PATH` | /tmp/marketplace.sled | 数据库路径 |
| `LOCAL_DATA_PATH` | ./data | 本地数据目录 |
| `RUST_LOG` | info | 日志级别 |

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
  -v ./data:/app/data \
  tinyiothub-marketplace
```

### 多架构构建

```bash
docker buildx build --platforms linux/amd64,linux/arm64 -t tinyiothub-marketplace --push .
```

## 项目结构

```
.
├── api/                    # Rust API 服务
│   └── src/
│       ├── api/v1/         # API 处理器
│       │   ├── templates.rs
│       │   └── drivers.rs
│       ├── domain/         # 领域模型
│       │   ├── template.rs  # 模板定义
│       │   └── driver.rs    # 驱动定义
│       ├── dto/           # 数据传输对象
│       ├── infrastructure/ # 基础设施
│       │   └── cache.rs   # Sled 缓存
│       └── sync/          # 数据加载服务
├── data/                   # 本地数据
│   ├── templates/
│   └── drivers/
├── docker-compose.yml
└── Dockerfile
```

## 技术栈

- **框架**: Axum 0.8
- **运行时**: Tokio
- **数据库**: Sled（嵌入式）
