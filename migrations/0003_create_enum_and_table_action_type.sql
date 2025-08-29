-- Add migration script here
CREATE TYPE change_enum AS ENUM(
    'INC',
    'DEC',
    'NONE'
);

CREATE TABLE IF NOT EXISTS "public"."action_type"(
    "id" serial PRIMARY KEY,
    "name" text UNIQUE NOT NULL,
    "description" text NOT NULL DEFAULT '',
    "available_balance_change" change_enum NOT NULL,
    "frozen_balance_change" change_enum NOT NULL,
    "total_income_change" change_enum NOT NULL,
    "total_expense_change" change_enum NOT NULL,
    "is_active" boolean NOT NULL DEFAULT FALSE,
    "created_at" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN "public"."action_type"."id" IS '主键自增id';

COMMENT ON COLUMN "public"."action_type"."name" IS '操作类型名称';

COMMENT ON COLUMN "public"."action_type"."description" IS '操作类型描述';

COMMENT ON COLUMN "public"."action_type"."available_balance_change" IS '账户可用余额变化';

COMMENT ON COLUMN "public"."action_type"."frozen_balance_change" IS '账户冻结余额变化';

COMMENT ON COLUMN "public"."action_type"."total_income_change" IS '账户累计收入变化';

COMMENT ON COLUMN "public"."action_type"."total_expense_change" IS '账户累计支出变化';

COMMENT ON COLUMN "public"."action_type"."is_active" IS '是否启用';

COMMENT ON COLUMN "public"."action_type"."created_at" IS '创建时间';

COMMENT ON COLUMN "public"."action_type"."updated_at" IS '更新时间';

COMMENT ON TABLE "public"."action_type" IS '账户操作类型表';
