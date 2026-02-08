# 卡片描述编辑功能

## 功能概述

在卡片详情模态框中添加了描述编辑功能，用户可以查看和编辑卡片描述。

## UI 设计

### 显示模式（默认）
```
┌─────────────────────────────────────┐
│ 描述                          [编辑] │
├─────────────────────────────────────┤
│ 这是卡片的描述内容...              │
│                                     │
└─────────────────────────────────────┘
```

### 编辑模式
```
┌─────────────────────────────────────┐
│ 描述                          [编辑] │
├─────────────────────────────────────┤
│ ┌─────────────────────────────────┐ │
│ │ 这是卡片的描述内容...          │ │
│ │                                 │ │
│ │ (可编辑的文本输入框)            │ │
│ └─────────────────────────────────┘ │
│                                     │
│ [保存]  [取消]                      │
└─────────────────────────────────────┘
```

## 组件结构

### card_info_section.rs

```rust
// 描述显示区域
card_description_label = <Label> {
    text: "暂无描述"
}

// 描述编辑区域（默认隐藏）
description_edit_container = <View> {
    visible: false,
    
    card_description_input = <TextInput> {
        height: 80,  // 多行输入框
    }
    
    save_description_button = <Button> {
        text: "保存"
    }
    
    cancel_description_button = <Button> {
        text: "取消"
    }
}
```

## 功能流程

### 1. 查看描述
```
打开卡片详情
  ↓
fetch_card_detail()
  ↓
handle_card_detail_signal()
  ↓
更新 card_description_label
  ↓
显示描述内容
```

### 2. 编辑描述
```
点击"编辑"按钮
  ↓
显示 description_edit_container
  ↓
填充当前描述到 card_description_input
  ↓
用户修改描述
  ↓
点击"保存"按钮
  ↓
update_card_description()
  ↓
API: PUT /api/v1/card/{id}
  ↓
handle_card_update_signal()
  ↓
刷新卡片数据
  ↓
隐藏编辑区域
```

### 3. 取消编辑
```
点击"取消"按钮
  ↓
隐藏 description_edit_container
  ↓
不保存修改
```

## 代码实现

### 按钮事件处理 (app.rs)

```rust
// 处理编辑按钮
if self.ui.button(id!(edit_description_button)).clicked(&actions) {
    // 显示编辑区域
    self.ui.view(id!(description_edit_container)).set_visible(cx, true);
    
    // 填充当前描述
    if let Some(card_detail) = &self.state.card_detail_data {
        let description = card_detail.description.as_deref().unwrap_or("");
        self.ui.text_input(id!(card_description_input)).set_text(cx, description);
    }
}

// 处理保存按钮
if self.ui.button(id!(save_description_button)).clicked(&actions) {
    let description = self.ui.text_input(id!(card_description_input)).text();
    
    if let Some(card_detail) = &self.state.card_detail_data {
        self.update_card_description(card_detail.id, description);
        self.ui.view(id!(description_edit_container)).set_visible(cx, false);
    }
}

// 处理取消按钮
if self.ui.button(id!(cancel_description_button)).clicked(&actions) {
    self.ui.view(id!(description_edit_container)).set_visible(cx, false);
}
```

### API 调用 (services/api.rs)

```rust
pub fn update_card_description(
    card_id: i64, 
    new_description: String, 
    tx: Sender<bool>, 
    signal: SignalToUI
) {
    std::thread::spawn(move || {
        let update_card = UpdateCardRequest {
            title: String::new(),
            description: Some(new_description),
            status: None,
        };

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/card/{}", Self::BASE_URL, card_id);
        
        let request = client
            .put(&url)
            .header("Content-Type", "application/json")
            .json(&update_card)
            .send();

        match request {
            Ok(response) => {
                let success = response.status().is_success();
                let _ = tx.send(success);
            }
            Err(_) => {
                let _ = tx.send(false);
            }
        }

        signal.set();
    });
}
```

## API 接口

### 更新卡片

```
PUT http://localhost:8911/api/v1/card/{id}
Content-Type: application/json

{
  "title": "",
  "description": "新的描述内容",
  "status": null
}
```

**说明**：
- `title` 为空字符串时，后端会保持原标题不变
- `description` 为新的描述内容
- `status` 为 null 时，后端会保持原状态不变

## 用户体验

### 优点
1. **即时编辑**：点击编辑按钮立即进入编辑模式
2. **数据回显**：编辑时自动填充当前描述
3. **可取消**：提供取消按钮，避免误操作
4. **视觉反馈**：编辑区域和显示区域分离，状态清晰

### 改进建议
1. 添加保存中的加载状态
2. 添加保存成功/失败的提示
3. 支持 Markdown 格式
4. 添加字数统计

## 测试步骤

1. 打开卡片详情模态框
2. 查看描述区域，应显示当前描述或"暂无描述"
3. 点击"编辑"按钮
4. 输入框应显示当前描述内容
5. 修改描述内容
6. 点击"保存"按钮
7. 描述应更新并返回显示模式
8. 或点击"取消"按钮，放弃修改

## 调试日志

```
点击编辑描述按钮
update_card_description: 更新卡片 2 的描述
API: 更新卡片描述 URL: http://localhost:8911/api/v1/card/2
API: 更新卡片描述响应状态: 200 OK
handle_card_update_signal: 收到结果: true
```
