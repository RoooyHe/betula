# Betula - Kanban 看板管理应用

基于 Makepad Framework (Rust) 和 Spring Boot 构建的 Kanban 看板管理应用。

## 技术栈

| 组件 | 技术栈 |
|------|--------|
| **前端 (Betula)** | Rust 2024 + Makepad Framework + reqwest |
| **后端 (Kankan)** | Java 25 + Spring Boot 4.0.2 + JPA + H2 + Lombok |
| **构建工具** | Cargo (Rust), Maven (Java) |
| **目标平台** | Windows (win32) |

## 快速开始

### 1. 启动后端服务 (Kankan)

```powershell
# 方式一：使用启动脚本
start-backend.cmd

# 方式二：手动启动
cd kankan
.\mvnw.cmd spring-boot:run
```

后端服务将在 `http://localhost:8911` 启动

### 2. 启动前端应用 (Betula)

```powershell
# 方式一：使用启动脚本
start-frontend.cmd

# 方式二：手动启动
cargo run

# 或者 Release 模式
cargo run --release
```

## API 接口

### 主要端点

- `GET /api/v1/space/byUserId/{userId}` - 获取用户空间列表（包含卡片和标签）
- `GET /api/v1/space` - 获取所有空间
- `POST /api/v1/space` - 创建新空间
- `PUT /api/v1/space/{id}` - 更新空间
- `DELETE /api/v1/space/{id}` - 删除空间

### 管理界面

- **API 基础路径**: `http://localhost:8911/api/v1`
- **H2 数据库控制台**: `http://localhost:8911/h2-console`
  - JDBC URL: `jdbc:h2:file:./data/kankan`
  - 用户名: `sa`
  - 密码: (空)

## 项目结构

```
betula/
├── src/                  # Betula 前端代码
│   ├── main.rs          # 程序入口
│   ├── app.rs           # 主应用逻辑和 UI 定义
│   ├── lib.rs           # 库定义
│   └── components/      # UI 组件
│       ├── mod.rs       # 组件模块导出
│       ├── space.rs     # 空间组件 (横向列表)
│       ├── card_list.rs # 卡片列表组件
│       ├── card_item.rs # 卡片项组件
│       ├── card_tag.rs  # 标签组件
│       └── card_modal.rs # 卡片弹窗组件
├── kankan/               # Kankan 后端服务
│   ├── src/main/java/com/roy/kankan/
│   │   ├── controller/   # REST 控制器
│   │   ├── entity/       # JPA 实体类
│   │   ├── repository/   # 数据访问层
│   │   └── ...
│   ├── src/main/resources/
│   │   ├── application.properties # 应用配置
│   │   └── data.sql      # 初始化数据
│   └── data/             # H2 数据库文件
├── start-backend.cmd     # 后端启动脚本
├── start-frontend.cmd    # 前端启动脚本
├── Cargo.toml           # Rust 项目配置
└── README.md
```

## 组件架构

### 前端组件层次

```
App
└── Space (横向空间列表)
    └── CardList (单个空间列)
        └── CardItem (卡片项)
            └── CardTag (标签)
```

### 数据流

```
App (启动事件)
    ↓
start_space_fetch() (异步 API 调用)
    ↓
handle_space_signal() (接收数据信号)
    ↓
apply_spaces() (更新共享状态)
    ↓
draw_walk() (组件渲染)
```

## 开发指南

### Rust/Makepad 开发规范

- 使用 `live_design!` 宏定义 UI DSL
- 遵循 Makepad 事件驱动架构
- 使用 `#[derive(Live, LiveHook, Widget)]` 定义组件
- 使用 `#[rust]` 标记 Rust 字段
- API 调用在独立线程中使用 `reqwest::blocking::get`
- 信号通知: `SignalToUI` + `Event::Signal`

### 示例数据

项目包含预置的示例数据：
- 3 个空间：Trello 新手指南、开发任务、设计工作
- 8 张卡片：涵盖开发、设计、测试等任务
- 6 个标签：非紧急、进行中、高优先级、设计、开发、测试

## 系统要求

- **Windows 10+** (win32 平台)
- **JDK 25+** (后端)
- **Rust 2024 Edition** (前端)
- **Maven 3.6+** (构建工具)

## 故障排除

1. **后端启动失败**: 检查 JDK 版本是否为 25+
2. **前端连接失败**: 确保后端服务在 8911 端口正常运行
3. **数据库问题**: 检查 `kankan/data/` 目录权限
4. **编译错误**: 运行 `cargo check` 检查依赖

## 许可证

MIT OR Apache-2.0
