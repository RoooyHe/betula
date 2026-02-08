# App.rs 代码结构说明

## 文件概述

`src/app.rs` 是 Betula 应用的主入口文件，包含了应用的 UI 定义和所有业务逻辑。

## 代码结构

### 1. 导入和 UI 定义 (第 1-985 行)

- **导入模块**: chrono, makepad_widgets, models, services
- **live_design! 宏**: 定义整个应用的 UI 结构
  - 主窗口布局
  - 空间列表 (SpaceList)
  - 卡片详情模态框 (card_detail_modal)
  - 标签管理 UI
  - Todo 管理 UI
  - Active 管理 UI

### 2. App 结构体定义 (第 986-990 行)

```rust
#[derive(Live)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}
```

### 3. 业务逻辑方法 (第 991-1740 行)

#### 3.1 Space 相关方法 (第 991-1118 行)

| 方法名 | 功能 | 说明 |
|--------|------|------|
| `start_space_fetch()` | 获取空间列表 | 调用 API 获取用户的所有空间 |
| `handle_space_signal()` | 处理空间数据信号 | 接收并处理 API 返回的空间数据 |
| `start_create_space()` | 创建新空间 | 调用 API 创建新空间 |
| `handle_create_space_signal()` | 处理创建空间信号 | 处理空间创建结果 |
| `update_space_title()` | 更新空间标题 | 调用 API 更新空间标题 |
| `handle_space_update_signal()` | 处理空间更新信号 | 处理空间更新结果 |

#### 3.2 Card 相关方法 (第 1119-1529 行)

| 方法名 | 功能 | 说明 |
|--------|------|------|
| `delete_card()` | 删除卡片 | 调用 API 删除指定卡片 |
| `fetch_card_detail()` | 获取卡片详情 | 获取卡片的完整信息 |
| `fetch_all_tags()` | 获取所有标签 | 获取系统中的所有标签 |
| `create_card_from_input()` | 从输入创建卡片 | 根据用户输入创建新卡片 |
| `handle_card_signal()` | 处理卡片信号 | 处理卡片操作结果 |
| `update_card_title()` | 更新卡片标题 | 调用 API 更新卡片标题 |
| `handle_card_update_signal()` | 处理卡片更新信号 | 处理卡片更新结果 |
| `handle_card_detail_signal()` | 处理卡片详情信号 | 更新模态框显示卡片详情 |
| `update_todo_items()` | 更新 Todo 项显示 | 在 UI 中显示 Todo 列表 |
| `handle_tags_signal()` | 处理标签信号 | 处理标签数据 |
| `update_tag_buttons()` | 更新标签按钮 | 在 UI 中显示标签按钮 |
| `handle_card_tags_update_signal()` | 处理卡片标签更新信号 | 处理标签更新结果 |
| `add_tag_to_card()` | 为卡片添加标签 | 调用 API 添加标签 |
| `create_new_tag()` | 创建新标签 | 调用 API 创建新标签 |
| `handle_create_tag_signal()` | 处理创建标签信号 | 处理标签创建结果 |

#### 3.3 Todo 相关方法 (第 1530-1652 行)

| 方法名 | 功能 | 说明 |
|--------|------|------|
| `create_new_todo()` | 创建新 Todo | 调用 API 创建待办事项 |
| `toggle_todo()` | 切换 Todo 状态 | 切换待办事项的完成状态 |
| `delete_todo()` | 删除 Todo | 调用 API 删除待办事项 |
| `handle_create_todo_signal()` | 处理创建 Todo 信号 | 处理 Todo 创建结果 |
| `handle_update_todo_signal()` | 处理更新 Todo 信号 | 处理 Todo 更新结果 |
| `handle_delete_todo_signal()` | 处理删除 Todo 信号 | 处理 Todo 删除结果 |

#### 3.4 Active 相关方法 (第 1653-1740 行)

| 方法名 | 功能 | 说明 |
|--------|------|------|
| `create_new_active()` | 创建新活动记录 | 调用 API 创建活动记录 |
| `delete_active()` | 删除活动记录 | 调用 API 删除活动记录 |
| `update_active_items()` | 更新活动记录显示 | 在 UI 中显示活动记录列表 |
| `handle_create_active_signal()` | 处理创建活动记录信号 | 处理活动记录创建结果 |
| `handle_delete_active_signal()` | 处理删除活动记录信号 | 处理活动记录删除结果 |

### 4. Trait 实现 (第 1741-2126 行)

#### 4.1 LiveRegister (第 1741-1747 行)

注册 Makepad 组件和样式。

#### 4.2 LiveHook (第 1749-1753 行)

在文档加载后自动获取空间数据。

#### 4.3 AppMain (第 1755-1893 行)

主事件处理循环：
- 处理启动事件
- 处理信号事件（所有 API 响应）
- 处理键盘事件
- 处理待处理的操作（pending operations）

#### 4.4 MatchEvent (第 1895-2124 行)

处理所有 UI 交互事件：
- 按钮点击
- 输入框变化
- 模态框操作
- 标签、Todo、Active 的 CRUD 操作

## 数据流

```
用户操作 (UI Event)
    ↓
MatchEvent::handle_actions() 
    ↓
调用相应的方法 (如 create_new_active)
    ↓
ApiService 在新线程中调用 API
    ↓
通过 SignalToUI 发送信号
    ↓
AppMain::handle_event() 接收 Event::Signal
    ↓
调用相应的 handle_*_signal() 方法
    ↓
更新 State 并刷新 UI (cx.redraw_all())
```

## 重构建议

### 已完成
✅ 添加清晰的注释分隔符
✅ 按功能模块组织方法

### 未来改进
- [ ] 将 UI 定义拆分到单独的文件
- [ ] 将 Active 相关方法提取到独立模块
- [ ] 将 Todo 相关方法提取到独立模块
- [ ] 将 Tag 相关方法提取到独立模块
- [ ] 减少重复代码（signal 处理模式相似）
- [ ] 考虑使用宏简化 signal 处理代码

## 注意事项

1. **线程安全**: 所有 API 调用都在独立线程中执行
2. **信号机制**: 使用 `SignalToUI` 和 `mpsc::channel` 进行线程间通信
3. **状态管理**: 所有状态存储在 `State` 结构体中
4. **UI 更新**: 通过 `cx.redraw_all()` 触发重绘
5. **错误处理**: 目前主要通过 `println!` 输出日志

## 相关文件

- `src/models/mod.rs` - State 结构体定义
- `src/models/dto.rs` - 数据传输对象定义
- `src/services/api.rs` - API 服务实现
- `src/components/` - UI 组件实现
