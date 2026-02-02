# Betula - Kanban 看板管理应用

一个基于 Makepad Framework 的 Kanban 看板管理应用，配合 Spring Boot 后端服务。

## 项目特性

- **Betula** - Rust + Makepad UI 前端
- **Kankan** - Java 25 + Spring Boot 后端服务
- **Makepad** - Rust 跨平台 UI 框架

## 快速开始

### 环境要求

- Rust 2024 Edition
- Java 25+
- Maven
- Windows 10+

### 运行后端服务

```powershell
cd kankan
.\mvnw.cmd spring-boot:run
```

后端服务运行在 http://localhost:8911

### 运行前端应用

```powershell
cargo run
```

## 项目结构

```
betula/
├── src/              # Betula Rust 前端
├── resources/        # 静态资源
├── kankan/           # Spring Boot 后端
│   ├── src/main/
│   └── data/         # H2 数据库
└── makepad/          # Makepad UI 框架
```

## 技术栈

| 部分 | 技术 |
|------|------|
| 前端 | Rust + Makepad |
| 后端 | Java 25 + Spring Boot 4.0.2 |
| 数据库 | H2 文件数据库 |
| 构建 | Cargo + Maven |

## License

MIT OR Apache-2.0
