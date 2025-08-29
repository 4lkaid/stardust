# Migrations

## 数据完整性保障

系统通过 PostgreSQL 的高级功能确保 `asset_type` 和 `action_type` 配置表的数据完整性。所有对这些表的操作（插入、更新、删除）都会自动记录到 `change_log` 表中，实现完整的数据变更追踪和审计功能。

## 分区管理策略

`account_log` 表采用按月分区策略，显著提升查询性能和数据管理效率。

#### 分区管理函数

**create_account_log_partition(month_date timestamptz DEFAULT CURRENT_TIMESTAMP)**

- 参数：month_date（可选，默认为当前月份）
- 功能：创建指定月份的分区表

**create_future_partitions(months_ahead int DEFAULT 3)**

- 参数：months_ahead（可选，默认3个月）
- 功能：自动创建未来数月的分区表

#### 使用示例

```sql
-- 创建指定月份分区
SELECT create_account_log_partition('2025-01-01'::timestamptz);

-- 创建未来5个月分区
SELECT create_future_partitions(5);
```

#### 初始分区设置

迁移脚本执行后自动创建：

- 当前月份的分区表
- 未来5个月的分区表

#### 维护建议

建议通过定时任务（如 cron job）定期执行分区创建函数，确保系统始终有足够的分区表可用。

## 重要注意事项

#### 配置变更处理

- 当修改 `asset_type` 和 `action_type` 表数据后，必须**重启应用服务**以使配置变更生效

#### 时区处理规范

- 所有时间字段统一使用 `timestamptz` 类型
- 分区策略基于 UTC 时间按月划分，确保数据分布的一致性
- 采用 UTC 时区作为存储标准，避免时区差异导致的数据问题
- 应用层负责处理时区转换和显示，保证用户体验

## 数据模型说明

#### 核心数据表

- **asset_type** - 资产类型配置
- **action_type** - 账户操作类型配置
- **account** - 用户资产账户
- **account_log** - 账户操作日志（按月分区）
- **change_log** - 系统数据变更审计日志

#### 枚举类型定义

`change_enum` 枚举值说明：

- `INC` 数值增加
- `DEC` 数值减少
- `NONE` 数值无变化

#### 账户余额关系

可用余额 + 冻结余额 = 账户总额

#### 操作日志金额字段说明

`account_log` 表中 `amount_x` 字段与 `account` 表中对应字段的关系：

- **正数值**表示对应账户字段增加
- **负数值**表示对应账户字段减少
- **零值**表示对应账户字段无变化

（其中 x 代表 available_balance、frozen_balance、total_income 或 total_expense）
