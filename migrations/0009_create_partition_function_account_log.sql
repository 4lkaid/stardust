-- Add migration script here
-- 账户日志表按月分区管理函数
CREATE OR REPLACE FUNCTION create_account_log_partition(month_date timestamptz DEFAULT CURRENT_TIMESTAMP)
    RETURNS void
    LANGUAGE plpgsql
    AS $$
DECLARE
    partition_name text;
    month_start timestamptz;
    month_end timestamptz;
    utc_month_start timestamptz;
    utc_month_end timestamptz;
BEGIN
    utc_month_start := date_trunc('month', month_date AT TIME ZONE 'UTC') AT TIME ZONE 'UTC';
    utc_month_end := utc_month_start + INTERVAL '1 month';
    partition_name := 'account_log_' || to_char(utc_month_start AT TIME ZONE 'UTC', 'YYYYMM');
    IF EXISTS (
        SELECT
            1
        FROM
            pg_tables
        WHERE
            schemaname = 'public'
            AND tablename = partition_name) THEN
    RAISE NOTICE '分区已存在: %', partition_name;
    RETURN;
END IF;
    EXECUTE format('CREATE TABLE %I PARTITION OF account_log FOR VALUES FROM (%L) TO (%L)', partition_name, utc_month_start, utc_month_end);
    RAISE NOTICE '分区创建成功: % (UTC时间范围: % 至 %)', partition_name, utc_month_start, utc_month_end;
END;
$$;

-- 自动创建未来N个月分区的函数
CREATE OR REPLACE FUNCTION create_future_partitions(months_ahead int DEFAULT 3)
    RETURNS void
    LANGUAGE plpgsql
    AS $$
DECLARE
    i int;
    target_month timestamptz;
BEGIN
    FOR i IN 1..months_ahead LOOP
        target_month := CURRENT_TIMESTAMP +(i || ' months')::interval;
        PERFORM
            create_account_log_partition(target_month);
    END LOOP;
END;
$$;

-- 创建当前月份分区
SELECT
    create_account_log_partition();

-- 创建未来5个月的分区
SELECT
    create_future_partitions(5);
