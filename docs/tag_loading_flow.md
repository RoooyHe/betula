# 标签数据加载流程

## 当前实现

### 数据流

```
用户点击卡片"详情"按钮
    ↓
App.handle_event: pending_detail_card_id 被设置
    ↓
fetch_card_detail(card_id) - 获取卡片详情
    ↓
fetch_all_tags() - 同时获取所有标签
    ↓
handle_tags_signal() - 收到标签数据
    ↓
state.all_tags 更新
    ↓
update_tag_buttons(cx) - 更新标签按钮文本
    ↓
标签按钮显示真实数据
```

### 关键代码位置

#### 1. 获取标签数据 (app.rs)

```rust
fn fetch_card_detail(&mut self, card_id: i64) {
    // ... 获取卡片详情
    
    // 同时获取全部标签
    self.fetch_all_tags();
}

fn fetch_all_tags(&mut self) {
    let (tx, rx) = std::sync::mpsc::channel();
    let signal = self.state.tags_signal.clone();
    self.state.tags_rx = Some(rx);
    
    ApiService::fetch_all_tags(tx, signal);
}
```

#### 2. 处理标签数据 (app.rs)

```rust
fn handle_tags_signal(&mut self, cx: &mut Cx) {
    if !self.state.tags_signal.check_and_clear() {
        return;
    }
    
    let mut received_tags = Vec::new();
    if let Some(rx) = &self.state.tags_rx {
        while let Ok(tags) = rx.try_recv() {
            received_tags.push(tags);
        }
    }
    
    for tags in received_tags {
        self.state.all_tags = tags;
        self.update_tag_buttons(cx);
        cx.redraw_all();
    }
}
```

#### 3. 更新标签按钮 (app.rs)

```rust
fn update_tag_buttons(&mut self, cx: &mut Cx) {
    // 更新固定的 3 个标签按钮文本
    if self.state.all_tags.len() > 0 {
        self.ui.button(id!(tag_button_1)).set_text(cx, &self.state.all_tags[0].title);
    }
    if self.state.all_tags.len() > 1 {
        self.ui.button(id!(tag_button_2)).set_text(cx, &self.state.all_tags[1].title);
    }
    if self.state.all_tags.len() > 2 {
        self.ui.button(id!(tag_button_3)).set_text(cx, &self.state.all_tags[2].title);
    }
    
    cx.redraw_all();
}
```

#### 4. UI 定义 (tag_section.rs)

```rust
tag_button_1 = <Button> {
    width: Fill,
    height: 25,
    text: "标签1"  // 默认文本，会被 update_tag_buttons 更新
    draw_bg: { color: #45B7D1 }
    draw_text: {
        color: #FFFFFF
        text_style: { font_size: 12.0 }
    }
}
```

## 当前限制

### 固定数量
- 只显示前 3 个标签
- 如果数据库中有超过 3 个标签，其余标签不会显示

### 解决方案
使用 PortalList 实现动态渲染，可以显示任意数量的标签。

## API 接口

### 获取所有标签

```
GET http://localhost:8911/api/v1/tag/all
```

**响应示例：**
```json
[
  {
    "id": 1,
    "title": "非紧急",
    "color": null
  },
  {
    "id": 2,
    "title": "重要",
    "color": "#FF0000"
  },
  {
    "id": 3,
    "title": "待处理",
    "color": "#00FF00"
  }
]
```

## 测试步骤

1. 确保后端服务运行在 `http://localhost:8911`
2. 确保数据库中有标签数据
3. 运行前端应用：`cargo run`
4. 点击任意卡片的"详情"按钮
5. 点击"添加标签"按钮
6. 查看标签列表是否显示真实数据

## 调试日志

在控制台中可以看到以下日志：

```
fetch_all_tags: 开始获取全部标签
handle_tags_signal: 收到标签信号
handle_tags_signal: 收到 3 个标签
handle_tags_signal: 标签数据已更新，共 3 个标签
  - 标签: 非紧急 (ID: 1, 颜色: None)
  - 标签: 重要 (ID: 2, 颜色: Some("#FF0000"))
  - 标签: 待处理 (ID: 3, 颜色: Some("#00FF00"))
update_tag_buttons: 标签 1 - '非紧急' (ID: 1)
update_tag_buttons: 标签 2 - '重要' (ID: 2)
update_tag_buttons: 标签 3 - '待处理' (ID: 3)
update_tag_buttons: 已更新 3 个标签按钮
```

## 后续优化

### 使用 PortalList 动态渲染

将 `tag_section.rs` 中的固定按钮改为 PortalList：

```rust
existing_tags = <PortalList> {
    width: Fill,
    height: Fit,
    flow: Down,
    spacing: 3,

    TagButton = <Button> {
        width: Fill,
        height: 25,
        draw_bg: { color: #45B7D1 }
        draw_text: {
            color: #FFFFFF
            text_style: { font_size: 12.0 }
        }
    }
}
```

然后创建自定义 Widget 在 `draw_walk` 中渲染：

```rust
impl Widget for TagSection {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            let state = scope.data.get_mut::<State>().unwrap();
            
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, state.all_tags.len());
                
                while let Some(idx) = list.next_visible_item(cx) {
                    let tag_item = list.item(cx, idx, live_id!(TagButton));
                    tag_item.set_text(cx, &state.all_tags[idx].title);
                }
            }
        }
        DrawStep::done()
    }
}
```

这样就可以显示任意数量的标签了。
