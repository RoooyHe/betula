# Card API 更新说明

## 🎯 **需求分析**

根据你的要求，我们需要区分两种不同的 Card 数据返回格式：

1. **Space 根据用户 ID 获取**：只显示 card 的 tag（轻量级）
2. **Card 根据 ID 获取**：显示完整信息包括 active 和 todo（详细信息）

## ✅ **已实现的解决方案**

### 1. 创建了 CardDetailDto
```java
// kankan/src/main/java/com/roy/kankan/dto/CardDetailDto.java
@Data
public class CardDetailDto {
    private Long id;
    private String title;
    private String description;
    private Boolean status;
    private LocalDateTime endTime;
    private List<TagEntity> tags;
    private List<TodoEntity> todos;      // 新增
    private List<ActiveEntity> active;   // 新增
}
```

### 2. 新增 Card 详情接口
```java
// 新增的端点
@GetMapping("/{id}/detail")
public ResponseEntity<CardDetailDto> getCardDetailById(@PathVariable Long id)
```

## 📋 **API 接口对比**

### 原有接口（保持不变）
```
GET /api/v1/space/byUserId/{userId}
```
**返回**：Space 列表，每个 Card 只包含基本信息 + tags

```json
{
  "cards": [
    {
      "id": 1,
      "title": "完成代码编辑",
      "description": "...",
      "status": false,
      "endTime": "...",
      "tags": [{"id": 1, "title": "非紧急"}]
      // 不包含 todos 和 active
    }
  ]
}
```

### 新增接口
```
GET /api/v1/card/{id}/detail
```
**返回**：Card 完整详情，包含 todos 和 active

```json
{
  "id": 1,
  "title": "完成代码编辑",
  "description": "...",
  "status": false,
  "endTime": "...",
  "tags": [{"id": 1, "title": "非紧急"}],
  "todos": [
    {"id": 1, "title": "编写测试用例", "completed": false},
    {"id": 2, "title": "代码审查", "completed": true}
  ],
  "active": [
    {"id": 1, "title": "正在开发功能A", "startTime": "..."}
  ]
}
```

### 原有 Card 接口（保持不变）
```
GET /api/v1/card/{id}
```
**返回**：Card 基本信息，不包含 todos 和 active

## 🔧 **技术实现**

### 1. 保持 CardEntity 的 @JsonIgnore
```java
@JsonIgnore
@OneToMany(mappedBy = "card", cascade = CascadeType.ALL, fetch = FetchType.LAZY)
private List<TodoEntity> todos;

@JsonIgnore
@OneToMany(mappedBy = "card", cascade = CascadeType.ALL, fetch = FetchType.LAZY)
private List<ActiveEntity> active;
```

### 2. 使用 DTO 转换
```java
public static CardDetailDto fromEntity(CardEntity card) {
    // 手动设置所有字段，包括 todos 和 active
}
```

## 🚀 **使用方式**

### 前端调用示例

```rust
// 获取空间列表（轻量级）
GET http://localhost:8911/api/v1/space/byUserId/1

// 获取卡片详情（包含 todos 和 active）
GET http://localhost:8911/api/v1/card/1/detail
```

## 📝 **优势**

1. **性能优化**：Space 查询时不加载不必要的 todos 和 active 数据
2. **灵活性**：可以根据需要选择不同的数据详细程度
3. **向后兼容**：不影响现有的 API 接口
4. **清晰分离**：不同的使用场景使用不同的端点

## 🔄 **下一步**

1. 启动后端服务测试新接口
2. 在前端添加调用新接口的功能
3. 根据需要在 UI 中显示 todos 和 active 信息

**状态**: ✅ 后端代码已完成，等待测试