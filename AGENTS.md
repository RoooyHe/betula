# AGENTS.md - Betula 项目上下文

## 项目概述

这是一个基于 Makepad Framework 的 Kanban 看板管理应用，包含 Rust 前端和 Spring Boot 后端。

## 技术栈

| 子项目 | 技术栈 |
|--------|--------|
| **Betula** | Rust 2024 + Makepad (Rust UI 框架) + reqwest |
| **Kankan** | Java 25 + Spring Boot 4.0.2 + JPA + H2 + Lombok |
| **构建工具** | Cargo (Rust), Maven (Java) |
| **目标平台** | Windows (win32) |

## 目录结构

```
betula/
├── src/                  # Betula 主应用代码
│   ├── main.rs          # 程序入口
│   ├── app.rs           # 应用主逻辑和 UI 定义
│   ├── lib.rs           # 库定义
│   └── components/      # UI 组件
│       ├── mod.rs       # 组件模块导出
│       ├── space.rs     # 空间组件 (横向列表)
│       ├── card_list.rs # 卡片列表组件
│       ├── card_item.rs # 卡片项组件
│       ├── card_tag.rs  # 标签组件
│       └── card_modal.rs # 卡片弹窗组件
├── kankan/               # Spring Boot 后端服务
│   ├── src/main/
│   │   ├── java/com/roy/kankan/
│   │   │   ├── controller/
│   │   │   │   ├── SpaceController.java    # 空间 API
│   │   │   │   ├── CardController.java     # 卡片 API
│   │   │   │   ├── TagController.java      # 标签 API
│   │   │   │   └── ...
│   │   │   ├── entity/
│   │   │   ├── repository/
│   │   │   └── ...
│   │   └── resources/
│   │       └── application.properties
│   └── data/             # H2 数据库文件
├── Cargo.toml            # Rust 配置
└── README.md
```

## API 接口

### 获取用户空间列表

```
GET http://localhost:8911/api/v1/space/byUserId/{userId}
```

**响应数据结构：**

```json
[
  {
    "id": 1,
    "title": "Trello 新手指南",
    "userId": "1",
    "canceled": false,
    "sort": 1,
    "color": "",
    "sortBy": "",
    "cards": [
      {
        "id": 2,
        "title": "完成代码编辑",
        "description": null,
        "status": false,
        "endTime": null,
        "tags": [
          { "id": 1, "title": "非紧急", "color": null }
        ]
      }
    ]
  }
]
```

**DTO 定义 (src/components/space.rs):**

```rust
#[derive(Clone, Deserialize)]
pub struct TagDto {
    pub id: i64,
    pub title: String,
    pub color: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct CardDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    pub end_time: Option<String>,
    pub tags: Vec<TagDto>,
}

#[derive(Clone, Deserialize)]
pub struct SpaceDto {
    pub id: i64,
    pub title: String,
    pub user_id: String,
    pub canceled: Option<bool>,
    pub sort: Option<i32>,
    pub color: Option<String>,
    pub sort_by: Option<String>,
    pub cards: Vec<CardDto>,
}
```

## 构建和运行

### Betula (Rust UI 应用)

```powershell
# Debug 构建
cargo run

# Release 构建
cargo run --release

# 检查依赖
cargo check
```

### Kankan (Spring Boot 后端服务)

```powershell
cd kankan

# 运行服务 (端口 8911)
.\mvnw.cmd spring-boot:run

# 打包
.\mvnw.cmd package
```

**后端服务配置:**
- API 基础路径: `http://localhost:8911/api/v1`
- H2 控制台: `http://localhost:8911/h2-console`
- 数据库: `jdbc:h2:file:./data/kankan`

## 组件架构

### Space (space.rs)
- 顶层容器，水平排列多个 CardList
- 启动时调用 `start_space_fetch()` 获取空间数据
- 使用 `PortalList` 实现动态空间列表

### CardList (card_list.rs)
- 单个空间列，包含标题和卡片列表
- 标题显示空间名称
- 使用 `PortalList` 渲染 CardItem

### CardItem (card_item.rs)
- 单个卡片项
- 显示卡片标题和标签
- 使用 `PortalList` 渲染 CardTag

### CardTag (card_tag.rs)
- 卡片标签组件
- 显示标签名称

### 数据流

```
App (handle_event Startup)
    ↓
start_space_fetch() (在新线程中请求 API)
    ↓
handle_space_signal() (收到 Signal)
    ↓
apply_spaces() (更新 spaces_data)
    ↓
draw_walk() (渲染时从 spaces_data 读取数据)
```

## 关键文件

| 文件 | 说明 |
|------|------|
| `src/app.rs` | 主应用入口，包含 UI 定义和事件处理 |
| `src/components/space.rs` | 空间组件 + DTO 定义 + API 调用 |
| `src/components/card_list.rs` | 卡片列表组件 |
| `src/components/card_item.rs` | 卡片项组件 |
| `src/components/card_tag.rs` | 标签组件 |
| `src/components/mod.rs` | 组件模块导出 |

## 开发规范

### Rust / Makepad

- 使用 `live_design!` 宏定义 UI DSL
- 遵循 Makepad 事件驱动架构
- 使用 `#[derive(Live, LiveHook, Widget)]` 定义组件
- 使用 `#[rust]` 标记 Rust 字段
- API 调用在独立线程中使用 `reqwest::blocking::get`
- 信号通知: `SignalToUI` + `Event::Signal`

### UI 定义示例

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

### 事件处理

```rust
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            self.start_space_fetch();
        }
        if let Event::Signal = event {
            self.handle_space_signal(cx);
        }
    }
}
```

## 注意事项

1. **Windows 环境**: 项目在 Windows 10+ 上开发测试
2. **Java 版本**: Kankan 需要 JDK 25+
3. **API 端口**: Kankan 后端服务运行在端口 8911
4. **线程安全**: API 调用在独立线程中执行，通过 channel 通信
5. **数据传递**: 使用 `spaces_data` 字段存储 API 返回数据