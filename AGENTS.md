# AGENTS.md - Betula 项目上下文

## 项目概述

这是一个包含多个子项目的工作空间，主要由以下部分组成：

1. **Betula** (`./`) - 基于 Makepad Framework 的 Kanban 看板管理 UI 应用
2. **Kankan** (`./kankan/`) - 基于 Spring Boot 的看板管理后端服务
3. **Makepad** (`./makepad/`) - 完整的 Rust UI 框架仓库（子模块）

## 技术栈

| 子项目 | 技术栈 |
|--------|--------|
| **Betula** | Rust 2024 + Makepad (Rust UI 框架) |
| **Kankan** | Java 25 + Spring Boot 4.0.2 + JPA + H2 + Lombok + MapStruct |
| **Makepad** | Rust + cargo-makepad (跨平台 UI 构建) |
| **构建工具** | Cargo (Rust), Maven (Java) |
| **目标平台** | Windows (win32), 支持 native 和 Web (wasm) |

## 目录结构

```
betula/
├── src/                  # Betula 主应用代码
│   ├── main.rs          # 程序入口，调用 betula::app::app_main()
│   ├── app.rs           # 应用主逻辑和 UI 定义 (~850 行)
│   ├── design.rs        # UI 设计/样式定义
│   └── lib.rs           # 库定义
├── resources/            # 静态资源文件
│   ├── left_arrow.svg   # 左箭头图标
│   ├── looking_glass.svg # 搜索图标
│   ├── placeholder.png  # 占位图
│   └── right_arrow.svg  # 右箭头图标
├── kankan/               # Spring Boot 后端服务
│   ├── src/main/
│   │   ├── java/com/roy/kankan/
│   │   │   ├── KankanApplication.java    # Spring Boot 入口
│   │   │   ├── command/                  # 命令模式实现
│   │   │   │   └── CreateTagCommand.java # 创建标签命令
│   │   │   ├── controller/               # REST API 控制器
│   │   │   │   ├── ActiveController.java
│   │   │   │   ├── CardController.java
│   │   │   │   ├── SpaceController.java
│   │   │   │   ├── TagController.java
│   │   │   │   └── TodoController.java
│   │   │   ├── entity/                   # JPA 实体类
│   │   │   │   ├── ActiveEntity.java
│   │   │   │   ├── CardEntity.java
│   │   │   │   ├── SpaceEntity.java
│   │   │   │   ├── TagEntity.java
│   │   │   │   └── TodoEntity.java
│   │   │   ├── repository/               # 数据访问层
│   │   │   │   ├── ActiveRepository.java
│   │   │   │   ├── CardRepository.java
│   │   │   │   ├── SpaceRepository.java
│   │   │   │   ├── TagRepository.java
│   │   │   │   └── TodoRepository.java
│   │   │   └── convert/                  # DTO 转换
│   │   │       └── TagConvert.java
│   │   └── resources/
│   │       ├── application.properties    # 服务配置
│   │       ├── static/                   # 静态资源
│   │       └── templates/                # 模板文件
│   ├── data/                   # H2 数据库文件
│   ├── pom.xml                 # Maven 配置
│   └── target/                 # 构建输出
├── makepad/                    # Makepad UI 框架仓库 (子模块)
│   ├── audio_graph/            # 音频处理模块
│   ├── code_editor/            # 代码编辑器组件
│   ├── draw/draw2/             # 2D/3D 绘图模块
│   ├── examples/               # 示例应用
│   ├── libs/                   # 基础库
│   ├── platform/               # 平台相关代码
│   ├── widgets/                # UI 组件库
│   └── tools/                  # 构建工具
└── Cargo.toml                  # Rust 工作空间配置
```

## 构建和运行

### Betula (Rust UI 应用)

```powershell
# Debug 构建
cargo run

# Release 构建
cargo run --release
```

### Kankan (Spring Boot 后端服务)

```powershell
cd kankan

# 运行服务 (默认端口 8911)
.\mvnw.cmd spring-boot:run

# 或打包后运行
.\mvnw.cmd package
java -jar target/kankan-0.0.1-SNAPSHOT.jar
```

**Kankan 服务配置** (端口 8911):
- REST API: `/api/*` (需查看各 Controller 定义)
- H2 数据库控制台: `http://localhost:8911/h2-console`
- 数据库: `jdbc:h2:file:./data/kankan`

### Makepad 示例

```powershell
cd makepad

# 运行 Makepad Studio
cargo run -p makepad-studio --release

# 运行特定示例
cargo run -p makepad-example-simple --release
```

## Betula 项目依赖

```toml
[dependencies]
makepad-widgets = "1.0.0"
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 开发规范

### Rust / Betula

- 使用 Rust 2024 Edition
- 使用 `live_design!` 宏定义 Makepad UI
- 遵循 Makepad 的事件驱动架构
- 使用 `makepad-widgets` 组件库
- 事件处理使用 `#[live]` 属性和 `#[derive(Live)]`

### UI 定义 (app.rs 示例)

```rust
live_design! {
    use link::theme::*;
    use link::widgets::*;
    
    App = {{App}} {
        ui: <Root> {
            <Window> {
                // 窗口配置
            }
        }
    }
}
```

### Java / Kankan

- 使用 Java 25
- 使用 Lombok 减少样板代码
- 使用 MapStruct 进行实体与 DTO 转换
- 使用 JPA/Hibernate 进行数据持久化
- 遵循 Spring Boot 约定优于配置原则
- 使用命令模式解耦业务逻辑 (command/ 目录)

## 关键文件说明

| 文件 | 说明 |
|------|------|
| `src/app.rs` | Betula 主应用，包含完整的 UI DSL 定义 (~850 行) |
| `src/main.rs` | Betula 入口文件，调用 `betula::app::app_main()` |
| `src/design.rs` | UI 设计/样式定义 |
| `kankan/pom.xml` | Kankan Maven 配置 (Spring Boot 4.0.2) |
| `kankan/src/main/java/com/roy/kankan/KankanApplication.java` | Kankan Spring Boot 入口 |
| `kankan/src/main/resources/application.properties` | Kankan 服务配置 |
| `kankan/src/main/java/com/roy/kankan/command/CreateTagCommand.java` | 命令模式示例 |
| `makepad/README.md` | Makepad 框架完整文档 |

## 常用命令

### Rust 命令

```powershell
# 检查依赖
cargo check

# 构建所有目标
cargo build --all-targets

# 运行测试
cargo test

# 更新依赖
cargo update
```

### Java/Maven 命令

```powershell
cd kankan

# 运行服务
.\mvnw.cmd spring-boot:run

# 编译打包
.\mvnw.cmd package

# 运行测试
.\mvnw.cmd test

# 清理构建
.\mvnw.cmd clean
```

## 注意事项

1. **Windows 环境**: 项目在 Windows 10+ 上开发测试
2. **Java 版本**: Kankan 需要 JDK 25+
3. **Makepad 依赖**: 需要安装 `cargo-makepad` 工具进行 wasm/移动端构建
4. **Web 构建**: Web 构建需要 nightly Rust 工具链
5. **数据库**: Kankan 使用 H2 文件数据库，首次运行会自动创建表结构
6. **H2 控制台**: 开发环境可访问 `/h2-console` 进行数据库管理
7. **API 端口**: Kankan 后端服务默认运行在端口 8911，Betula 前端需连接此端口
