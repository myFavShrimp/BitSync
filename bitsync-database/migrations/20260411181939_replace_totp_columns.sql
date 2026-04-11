ALTER TABLE "user"
    DROP COLUMN "totp_secret",
    DROP COLUMN "is_totp_set_up",
    ADD COLUMN "active_totp_secret" BYTEA,
    ADD COLUMN "dangling_totp_secret" BYTEA;
