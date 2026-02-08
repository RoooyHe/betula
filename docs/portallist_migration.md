# PortalList 迁移指南

## 已完成的更改

### 1. UI 定义更新 (`src/components/card_modal.rs`)

✅ **标签列表** - 从 10 个固定按钮改为 PortalList
```rust
existing_tags = <PortalList> {
    TagButton = <Button> { ... }
}
```

✅ **Todo 列表** - 从 5 个固定项改为 PortalList
```rust
existing_todos = <PortalList> {
    TodoItem = <View> {
        todo_check = <Button> { ... }
        todo_text = <Label> { ... }
        todo_delete = <Button> { ... }
    }
}
```

### 2. State 结构更新 (`src/models/state.rs`)

✅ 移除固定数组：
- ~~`tag_button_ids: [Option<i64>; 10]`~~
- ~~`todo_items: [Option<TodoDto>; 5]`~~
- ~~`active_items: [Option<ActiveDto>; 3]`~~

✅ 添加动态 Vec：
- `current_todos: Vec<TodoDto>`
- `current_actives: Vec<ActiveDto>`

### 3. 方法更新 (`src/app.rs`)

✅ `update_tag_buttons()` - 使用 PortalList API
```rust
let portal_list = self.ui.portal_list(id!(existing_tags));
portal_list.set_item_range(cx, 0, self.state.all_tags.len());
```

✅ `update_todo_items()` - 使用 PortalList API
```rust
let portal_list = self.ui.portal_list(id!(existing_todos));
portal_list.set_item_range(cx, 0, todos.len());
self.state.current_todos = todos.to_vec();
```

✅ Todo 切换事件处理 - 从 Vec 中查找
```rust
if let Some(todo) = self.state.current_todos.iter().find(|t| t.id == todo_id) {
    self.toggle_todo(todo_id, todo.title.clone());
}
```

## 需要完成的更改

### 1. PortalList 数据绑定

需要实现 `PortalList` 的 `draw_walk` 方法来渲染每个项：

```rust
// 在 impl MatchEvent for App 中添加
fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
    // ... 现有代码 ...
    
    // 处理标签按钮的 PortalList
    let portal_list = self.ui.portal_list(id!(existing_tags));
    while let Some(item_id) = portal_list.next_visible_item(cx) {
        if item_id < self.state.all_tags.len() {
            let tag = &self.state.all_tags[item_id];
            let item = portal_list.item(cx, item_id, live_id!(TagButton));
            
            item.button(id!(tag_button)).set_text(cx, &tag.title);
            
            // 处理点击事件
            if item.button(id!(tag_button)).clicked(actions) {
                if let Some(card_detail) = &self.state.card_detail_data {
                    self.add_tag_to_card(card_detail.id, tag.id);
                    self.ui.view(id!(tag_dropdown)).set_visible(cx, false);
                    cx.redraw_all();
                }
            }
        }
    }
    
    // 处理 Todo 的 PortalList
    let portal_list = self.ui.portal_list(id!(existing_todos));
    while let Some(item_id) = portal_list.next_visible_item(cx) {
        if item_id < self.state.current_todos.len() {
            let todo = &self.state.current_todos[item_id];
            let item = portal_list.item(cx, item_id, live_id!(TodoItem));
            
            // 设置复选框状态
            let check_text = if todo.completed.unwrap_or(false) { "✓" } else { "○" };
            item.button(id!(todo_check)).set_text(cx, check_text);
            
            // 设置文本
            item.label(id!(todo_text)).set_text(cx, &todo.title);
            
            // 处理复选框点击
            if item.button(id!(todo_check)).clicked(actions) {
                let current_completed = todo.completed.unwrap_or(false);
                self.state.pending_toggle_todo = Some((todo.id, !current_completed));
            }
            
            // 处理删除按钮点击
            if item.button(id!(todo_delete)).clicked(actions) {
                self.state.pending_delete_todo = Some(todo.id);
            }
        }
    }
}
```

### 2. 移除旧的固定按钮处理代码

需要删除 `impl MatchEvent` 中的以下代码：

```rust
// ❌ 删除这段代码
// 处理标签按钮点击
let button_ids = [
    id!(tag_btn_1), id!(tag_btn_2), id!(tag_btn_3), ...
];
for (index, button_id) in button_ids.iter().enumerate() {
    if self.ui.button(*button_id).clicked(&actions) {
        // ...
    }
}

// ❌ 删除这段代码
// 处理Todo复选框点击
let todo_check_ids = [
    id!(todo_check_1), id!(todo_check_2), ...
];
for (index, check_id) in todo_check_ids.iter().enumerate() {
    // ...
}

// ❌ 删除这段代码
// 处理Todo删除按钮点击
let todo_delete_ids = [
    id!(todo_delete_1), id!(todo_delete_2), ...
];
```

### 3. Active 列表迁移

同样需要为 Active 列表添加 PortalList 支持：

**card_modal.rs:**
```rust
// 添加 Active 的 PortalList 定义
existing_actives = <PortalList> {
    ActiveItem = <View> {
        active_text = <Label> { ... }
        active_time = <Label> { ... }
        active_delete = <Button> { ... }
    }
}
```

**app.rs:**
```rust
fn update_active_items(&mut self, cx: &mut Cx, actives: &[ActiveDto]) {
    let portal_list = self.ui.portal_list(id!(existing_actives));
    portal_list.set_item_range(cx, 0, actives.len());
    self.state.current_actives = actives.to_vec();
    cx.redraw_all();
}
```

## PortalList 的优势

1. **无数量限制** - 不再受固定数组大小限制
2. **性能优化** - 只渲染可见项
3. **代码简洁** - 不需要手动管理每个项的可见性
4. **动态扩展** - 自动适应数据变化

## 注意事项

1. **ID 命名** - PortalList 中的模板组件使用 `live_id!()` 宏
2. **事件处理** - 通过 `next_visible_item()` 迭代处理每个可见项的事件
3. **数据同步** - 确保 State 中的 Vec 与 UI 保持同步
4. **性能考虑** - PortalList 会自动优化渲染，只处理可见区域的项

## 参考示例

查看 `src/components/space.rs` 中 SpaceList 的实现，了解完整的 PortalList 使用模式。
