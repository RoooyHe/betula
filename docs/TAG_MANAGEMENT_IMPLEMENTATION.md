# 标签管理功能实现

## ✅ **功能概述**

在卡片详情模态框中添加了完整的标签管理功能，包括：
- 显示当前卡片的标签
- 添加标签按钮和下拉选择框
- 6个预定义标签选项
- 实时更新标签到后端
- 自动刷新界面显示

## 🔧 **技术实现**

### 1. **后端API支持**

已有的API接口：
- `GET /api/v1/tag` - 获取全部标签
- `PUT /api/v1/card/{id}` - 更新卡片（包括标签）

### 2. **前端数据结构**

#### DTO更新
```rust
// 添加了Serialize trait支持
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TagDto {
    pub id: i64,
    pub title: String,
    pub color: Option<String>,
}

// 新增卡片标签更新请求
#[derive(Serialize, Debug)]
pub struct UpdateCardTagsRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    pub tags: Vec<TagDto>,
}
```

#### State状态管理
```rust
// 标签管理相关状态
pub all_tags: Vec<TagDto>,
pub tags_signal: SignalToUI,
pub tags_rx: Option<Receiver<Vec<TagDto>>>,
pub selected_tag_id: Option<i64>,
pub pending_add_tag_to_card: Option<(i64, i64)>,
pub card_tags_update_signal: SignalToUI,
pub card_tags_update_rx: Option<Receiver<bool>>,
```

### 3. **API服务层**

#### 获取全部标签
```rust
pub fn fetch_all_tags(tx: Sender<Vec<TagDto>>, signal: SignalToUI) {
    // 异步请求 GET /api/v1/tag
    // 解析JSON响应为Vec<TagDto>
    // 通过channel发送结果
}
```

#### 更新卡片标签
```rust
pub fn update_card_tags(
    card_id: i64, 
    title: String, 
    description: Option<String>, 
    status: Option<bool>, 
    tags: Vec<TagDto>, 
    tx: Sender<bool>, 
    signal: SignalToUI
) {
    // 构造UpdateCardTagsRequest
    // 发送PUT请求到 /api/v1/card/{id}
    // 返回成功/失败状态
}
```

### 4. **UI界面设计**

#### 标签显示区域
```rust
<View> {
    // 标签标题
    <Label> { text: "标签" }
    
    <View> {
        // 当前标签显示
        card_tags = <Label> { text: "暂无标签" }
        
        // 添加标签按钮
        add_tag_button = <Button> { 
            text: "添加标签"
            color: #x4ECDC4
        }
    }
    
    // 标签下拉选择框
    tag_dropdown = <View> {
        visible: false,  // 默认隐藏
        
        // 6个标签选项按钮
        tag_option_1 = <Button> { text: "非紧急", color: #x9FE7B4 }
        tag_option_2 = <Button> { text: "进行中", color: #x61BD4F }
        tag_option_3 = <Button> { text: "高优先级", color: #xFF6B6B }
        tag_option_4 = <Button> { text: "设计", color: #x4ECDC4 }
        tag_option_5 = <Button> { text: "开发", color: #x45B7D1 }
        tag_option_6 = <Button> { text: "测试", color: #xFFA07A }
    }
}
```

### 5. **事件处理流程**

#### 打开卡片详情
1. 用户点击"详情"按钮
2. 调用`fetch_card_detail(card_id)`
3. 同时调用`fetch_all_tags()`获取标签列表
4. 显示模态框和卡片信息

#### 添加标签
1. 用户点击"添加标签"按钮
2. 切换下拉框显示/隐藏状态
3. 用户点击具体标签选项
4. 调用`add_tag_to_card(card_id, tag_id)`
5. 检查标签是否已存在
6. 构造新的标签列表
7. 调用API更新卡片
8. 刷新界面显示

#### 信号处理
```rust
Event::Signal => {
    self.handle_space_signal(cx);
    self.handle_create_space_signal(cx);
    self.handle_card_signal(cx);
    self.handle_space_update_signal(cx);
    self.handle_card_update_signal(cx);
    self.handle_card_detail_signal(cx);
    self.handle_tags_signal(cx);           // 新增
    self.handle_card_tags_update_signal(cx); // 新增
}
```

## 🎯 **用户交互流程**

1. **查看标签**：
   - 点击卡片"详情"按钮
   - 模态框显示当前卡片标签

2. **添加标签**：
   - 点击"添加标签"按钮
   - 下拉框显示6个标签选项
   - 点击选择标签
   - 自动添加到卡片并更新显示

3. **标签去重**：
   - 系统自动检查标签是否已存在
   - 避免重复添加相同标签

## 📊 **数据库标签**

系统预定义6个标签：
1. **非紧急** (#9FE7B4FF) - ID: 1
2. **进行中** (#61BD4FFF) - ID: 2  
3. **高优先级** (#FF6B6BFF) - ID: 3
4. **设计** (#4ECDC4FF) - ID: 4
5. **开发** (#45B7D1FF) - ID: 5
6. **测试** (#FFA07AFF) - ID: 6

## 🔄 **完整数据流**

```
用户点击"详情" 
    ↓
fetch_card_detail() + fetch_all_tags()
    ↓
显示模态框 + 标签信息
    ↓
用户点击"添加标签"
    ↓
显示标签下拉框
    ↓
用户选择标签
    ↓
add_tag_to_card()
    ↓
update_card_tags() API调用
    ↓
刷新界面显示
```

## ✅ **功能验证**

- ✅ 编译成功，无语法错误
- ✅ 模态框正常显示
- ✅ API接口调用正常
- ✅ 标签按钮和下拉框UI完整
- ✅ 事件处理链完整
- ✅ 数据结构支持序列化

## 🎉 **用户体验**

- **直观操作**：点击按钮即可添加标签
- **视觉反馈**：标签按钮使用对应颜色
- **防重复**：自动检查避免重复标签
- **实时更新**：添加后立即刷新显示
- **响应式**：下拉框自动隐藏

**状态**: 🟢 **实现完成，功能可用**