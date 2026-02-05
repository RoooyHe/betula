# Betula 代码重构计划

## 🔍 **当前问题分析**

### ❌ **主要问题**

1. **`src/app.rs` 职能过重 (750+ 行)**
   - UI 定义和布局
   - 应用状态管理
   - API 调用逻辑
   - 事件处理
   - 信号处理
   - 数据转换

2. **`src/components/space.rs` 混合职责**
   - UI 组件定义
   - DTO 数据结构定义
   - API 请求结构定义
   - 组件逻辑

3. **未使用的组件**
   - ✅ `card_modal.rs` - 已删除（改用内联编辑）
   - ✅ `card_tag.rs` - 已删除（集成到 card_item 中）

4. **缺失的抽象层**
   - 没有专门的 API 服务层
   - 没有数据模型层
   - 没有状态管理层

## 🔧 **重构方案**

### ✅ **已完成的重构**

1. **创建数据模型层** (`src/models/`)
   - `dto.rs` - 数据传输对象定义
   - `state.rs` - 应用状态结构
   - `mod.rs` - 模块导出

2. **创建服务层** (`src/services/`)
   - `api.rs` - API 调用服务
   - `mod.rs` - 模块导出

3. **清理未使用组件**
   - 删除 `card_modal.rs`
   - 删除 `card_tag.rs`
   - 更新 `components/mod.rs`

### 🚧 **待完成的重构**

#### 1. **重构 `src/app.rs`**

**当前结构：**
```rust
// app.rs (750+ 行)
- live_design! { ... }           // UI 定义
- struct App { ... }             // 应用结构
- impl App { ... }               // 所有业务逻辑
  - start_space_fetch()          // API 调用
  - handle_space_signal()        // 信号处理
  - create_card_from_input()     // 业务逻辑
  - update_space_title()         // API 调用
  - ... (20+ 方法)
- impl AppMain { ... }           // 事件处理
- impl MatchEvent { ... }        // 动作处理
- pub struct State { ... }       // 状态定义
```

**建议拆分为：**

```rust
// src/app.rs (简化为 ~200 行)
- live_design! { ... }           // UI 定义
- struct App { ... }             // 应用结构
- impl App { ... }               // 核心应用逻辑
- impl AppMain { ... }           // 事件处理
- impl MatchEvent { ... }        // 动作处理

// src/controllers/app_controller.rs (新建)
- struct AppController { ... }   // 应用控制器
- impl AppController { ... }     // 业务逻辑方法
  - handle_space_operations()
  - handle_card_operations()
  - handle_ui_events()

// src/models/state.rs (已创建)
- pub struct State { ... }       // 状态定义

// src/services/api.rs (已创建)
- struct ApiService { ... }      // API 服务
```

#### 2. **重构 `src/components/space.rs`**

**当前问题：**
- 混合了 UI 组件定义和 DTO 定义
- 包含了 API 请求结构

**建议拆分：**

```rust
// src/components/space.rs (保留)
- live_design! { ... }           // UI 定义
- struct SpaceColumn { ... }     // 组件结构
- struct SpaceList { ... }       // 组件结构
- impl Widget for SpaceColumn    // 组件逻辑
- impl Widget for SpaceList      // 组件逻辑

// src/models/dto.rs (已创建)
- struct TagDto { ... }          // DTO 定义
- struct CardDto { ... }         // DTO 定义
- struct SpaceDto { ... }        // DTO 定义
- struct CreateSpaceRequest { ... } // API 请求结构
- ... (其他 DTO)
```

#### 3. **创建控制器层**

```rust
// src/controllers/mod.rs (新建)
pub mod app_controller;
pub use app_controller::*;

// src/controllers/app_controller.rs (新建)
pub struct AppController;
impl AppController {
    // 空间相关操作
    pub fn fetch_spaces(state: &mut State) { ... }
    pub fn create_space(state: &mut State, title: String) { ... }
    pub fn update_space_title(state: &mut State, space_id: i64, title: String) { ... }
    
    // 卡片相关操作
    pub fn create_card(state: &mut State, space_id: i64, title: String) { ... }
    pub fn update_card_title(state: &mut State, card_id: i64, title: String) { ... }
    pub fn delete_card(state: &mut State, card_id: i64) { ... }
    
    // 信号处理
    pub fn handle_space_signal(state: &mut State, cx: &mut Cx) { ... }
    pub fn handle_card_signal(state: &mut State, cx: &mut Cx) { ... }
}
```

## 📁 **重构后的目录结构**

```
src/
├── main.rs                      # 程序入口
├── lib.rs                       # 库定义
├── app.rs                       # 简化的应用主逻辑 (~200 行)
├── components/                  # UI 组件层
│   ├── mod.rs                   # 组件模块导出
│   ├── space.rs                 # 空间组件 (纯 UI)
│   ├── card_list.rs             # 卡片列表组件 (纯 UI)
│   └── card_item.rs             # 卡片项组件 (纯 UI)
├── controllers/                 # 控制器层 (新建)
│   ├── mod.rs                   # 控制器模块导出
│   └── app_controller.rs        # 应用控制器
├── models/                      # 数据模型层 (已创建)
│   ├── mod.rs                   # 模型模块导出
│   ├── dto.rs                   # 数据传输对象
│   └── state.rs                 # 应用状态
└── services/                    # 服务层 (已创建)
    ├── mod.rs                   # 服务模块导出
    └── api.rs                   # API 服务
```

## 🎯 **重构的好处**

1. **职责分离**：每个文件和模块都有明确的职责
2. **可维护性**：代码更容易理解和修改
3. **可测试性**：各层可以独立测试
4. **可扩展性**：新功能更容易添加
5. **代码复用**：服务层可以在不同组件中复用

## 📋 **重构步骤**

### 阶段 1：创建基础结构 ✅
- [x] 创建 `models/` 目录和文件
- [x] 创建 `services/` 目录和文件
- [x] 删除未使用的组件
- [x] 更新 `lib.rs` 导入

### 阶段 2：重构 API 调用
- [ ] 将 `app.rs` 中的 API 调用方法移动到 `services/api.rs`
- [ ] 更新 `app.rs` 使用新的 API 服务
- [ ] 测试 API 功能是否正常

### 阶段 3：创建控制器层
- [ ] 创建 `controllers/` 目录
- [ ] 创建 `app_controller.rs`
- [ ] 将业务逻辑从 `app.rs` 移动到控制器
- [ ] 更新 `app.rs` 使用控制器

### 阶段 4：清理组件文件
- [ ] 从 `components/space.rs` 移除 DTO 定义
- [ ] 更新组件文件的导入语句
- [ ] 确保所有组件正常工作

### 阶段 5：测试和优化
- [ ] 全面测试所有功能
- [ ] 优化代码结构
- [ ] 更新文档

## ⚠️ **注意事项**

1. **渐进式重构**：一次只重构一个模块，确保功能正常
2. **保持功能完整**：重构过程中不改变现有功能
3. **测试驱动**：每次重构后都要测试功能是否正常
4. **备份代码**：重构前备份当前工作版本