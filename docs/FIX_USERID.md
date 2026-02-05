# 修复 userId 问题

## 问题描述
在更新空间标题时，由于前端没有传递 `userId`，导致某些空间的 `userId` 被设置为 `null`，现在查询时查不到这些空间。

## 已修复的代码问题
1. ✅ 更新了 `UpdateSpaceRequest` 包含 `userId` 字段
2. ✅ 更新了 `ApiService::update_space_title` 传递 `userId`

## 数据库修复方案

### 方案1：重启后端服务（推荐）
由于使用的是 H2 内存数据库，重启后端服务会重新加载 `data.sql`，恢复所有数据：

```powershell
# 停止后端服务（Ctrl+C）
# 然后重新启动
cd kankan
.\mvnw.cmd spring-boot:run
```

### 方案2：手动 SQL 修复
如果需要保留当前的修改，可以通过 H2 控制台执行：

```sql
-- 访问 http://localhost:8911/h2-console
-- 连接信息：
-- JDBC URL: jdbc:h2:file:./data/kankan
-- User Name: sa
-- Password: (空)

-- 修复 userId 为 null 的空间
UPDATE space SET user_id = '1' WHERE user_id IS NULL;

-- 检查修复结果
SELECT id, title, user_id FROM space WHERE user_id = '1';
```

## 验证修复
修复后应该能看到所有 12 个空间重新显示。

## 预防措施
现在前端代码已经修复，以后更新空间时会正确传递 `userId`，不会再出现这个问题。