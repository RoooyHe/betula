# 卡片详情模态框组件拆分

## 重构目标

将原本 600+ 行的 `card_modal.rs` 拆分成多个小的、功能明确的组件，提高代码可维护性和可读性。

## 组件结构

### 主组件
- **card_modal.rs** (60 行) - 卡片详情模态框主组件，组合所有子组件

### 子组件

#### 1. modal_header.rs (40 行)
**功能**: 模态框标题栏
- 显示"卡片详情"标题
- 关闭按钮（×）

**元素**:
- `<Label>` - 标题文本
- `close_button` - 关闭按钮

---

#### 2. card_info_section.rs (110 行)
**功能**: 卡片基本信息区域
- 卡片标题
- 卡片描述
- 卡片状态

**元素**:
- `card_title` - 标题 Label
- `card_description` - 描述 Label
- `card_status` - 状态 Label

---

#### 3. tag_section.rs (180 行)
**功能**: 标签管理区域
- 显示当前标签
- 添加标签按钮
- 标签下拉框（包含现有标签列表）
- 新增标签功能

**元素**:
- `card_tags` - 当前标签显示
- `add_tag_button` - 添加标签按钮
- `tag_dropdown` - 标签下拉框
- `tag_button_1/2/3` - 固定标签按钮（后续可改为 PortalList）
- `new_tag_button` - 新增标签按钮
- `new_tag_input_container` - 新增标签输入框容器
- `new_tag_input` - 新增标签输入框

---

#### 4. todo_section.rs (130 行)
**功能**: 待办事项管理区域
- 显示待办事项列表
- 添加待办按钮
- 待办下拉框
- 新增待办功能

**元素**:
- `card_todos` - 待办事项显示
- `add_todo_button` - 添加待办按钮
- `todo_dropdown` - 待办下拉框
- `new_todo_button` - 新增待办按钮
- `new_todo_input_container` - 新增待办输入框容器
- `new_todo_input` - 新增待办输入框

---

#### 5. active_section.rs (130 行)
**功能**: 活动记录区域
- 显示活动记录列表
- 添加活动按钮
- 活动下拉框
- 新增活动功能

**元素**:
- `card_active` - 活动记录显示
- `add_active_button` - 添加活动按钮
- `active_dropdown` - 活动下拉框
- `new_active_button` - 新增活动按钮
- `new_active_input_container` - 新增活动输入框容器
- `new_active_input` - 新增活动输入框

---

## 组件依赖关系

```
card_modal.rs
├── modal_header.rs
├── card_info_section.rs
├── tag_section.rs
├── todo_section.rs
└── active_section.rs
```

## 使用方式

### 在 card_modal.rs 中组合

```rust
live_design! {
    use crate::components::modal_header::ModalHeader;
    use crate::components::card_info_section::CardInfoSection;
    use crate::components::tag_section::TagSection;
    use crate::components::todo_section::TodoSection;
    use crate::components::active_section::ActiveSection;

    pub CardDetailModal = <Modal> {
        content: <RoundedView> {
            <ModalHeader> {}
            <ScrollYView> {
                <View> {
                    <CardInfoSection> {}
                    <TagSection> {}
                    <TodoSection> {}
                    <ActiveSection> {}
                }
            }
        }
    }
}
```

### 在 mod.rs 中注册

```rust
pub mod modal_header;
pub mod card_info_section;
pub mod tag_section;
pub mod todo_section;
pub mod active_section;

pub fn live_design(cx: &mut Cx) {
    // 先注册子组件
    modal_header::live_design(cx);
    card_info_section::live_design(cx);
    tag_section::live_design(cx);
    todo_section::live_design(cx);
    active_section::live_design(cx);
    
    // 最后注册主组件
    card_modal::live_design(cx);
}
```

## 优势

1. **代码可读性**: 每个文件只关注一个功能区域，代码更清晰
2. **可维护性**: 修改某个功能时只需要编辑对应的文件
3. **可复用性**: 子组件可以在其他地方复用
4. **团队协作**: 多人可以同时编辑不同的组件文件
5. **测试友好**: 可以单独测试每个组件

## 后续优化

1. **PortalList 集成**: 将 `tag_section.rs` 中的固定标签按钮改为 PortalList 动态渲染
2. **自定义 Widget**: 为需要复杂逻辑的组件（如 TagSection）创建自定义 Widget
3. **样式统一**: 提取公共样式到主题文件
4. **事件处理**: 在各个子组件中添加事件处理逻辑

## 文件大小对比

| 文件 | 重构前 | 重构后 |
|------|--------|--------|
| card_modal.rs | 600+ 行 | 60 行 |
| modal_header.rs | - | 40 行 |
| card_info_section.rs | - | 110 行 |
| tag_section.rs | - | 180 行 |
| todo_section.rs | - | 130 行 |
| active_section.rs | - | 130 行 |
| **总计** | **600+ 行** | **650 行** |

虽然总行数略有增加（因为增加了模块声明和导入），但每个文件都更小、更专注，大大提高了可维护性。
