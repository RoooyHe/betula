# 标签列表渲染问题修复

## 问题描述

卡片详情模态框中的标签列表无法正常显示，PortalList 无法渲染标签数据。

## 根本原因

CardDetailModal 的 Widget 结构定义缺少 `#[live]` 属性标记，导致 live_design 中的 `view: <Modal>` 字段无法正确映射到 Rust 结构体。

## 错误信息

```
Apply error: src/components/card_modal.rs:4:76 - no matching field: view
```

## 修复方案

### 修改前

```rust
#[derive(Live, LiveHook, Widget)]
pub struct CardDetailModal {
    #[deref]
    view: View,
}
```

### 修改后

```rust
#[derive(Live, LiveHook, Widget)]
pub struct CardDetailModal {
    #[deref]
    #[live]  // 添加 #[live] 属性
    view: View,
}
```

## 技术说明

在 Makepad Framework 中：

1. **`#[live]` 属性**：标记字段可以从 live_design DSL 中初始化
2. **`#[deref]` 属性**：允许 CardDetailModal 自动解引用到 View
3. **字段映射**：live_design 中的 `view: <Modal>` 需要映射到 Rust 结构体的 `view` 字段

没有 `#[live]` 属性时，Makepad 无法将 DSL 中的 `view` 字段映射到结构体，导致 "no matching field" 错误。

## 验证步骤

1. 编译检查：`cargo check` - 应该没有错误
2. 运行应用：`cargo run`
3. 点击卡片项的"详情"按钮
4. 点击"添加标签"按钮
5. 查看标签列表是否正常显示

## 相关文件

- `src/components/card_modal.rs` - CardDetailModal 组件定义
- `src/app.rs` - 模态框打开/关闭逻辑
- `src/models/state.rs` - 标签数据存储

## 数据流

```
fetch_all_tags() (API 调用)
    ↓
handle_tags_signal() (收到标签数据)
    ↓
state.all_tags (存储到 State)
    ↓
CardDetailModal.draw_walk() (渲染 PortalList)
    ↓
标签列表显示
```

## 注意事项

1. **PortalList 渲染**：标签列表使用 PortalList 动态渲染，数据存储在 `state.all_tags` 中
2. **调试日志**：`draw_walk` 方法中有详细的调试日志，可以查看渲染过程
3. **标签按钮点击**：标签按钮的点击事件处理仍需实现（标记为 TODO）

## 后续工作

1. 实现标签按钮点击事件处理
2. 为标签添加颜色显示
3. 支持从卡片中移除标签
