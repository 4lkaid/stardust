-- Add migration script here
INSERT INTO "public"."asset_type"("name", "description", "is_active")
    VALUES ('GOLD', '金币', 't'),
('SILVER', '银币', 't'),
('COPPER', '铜币', 't');
