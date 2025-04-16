CREATE TABLE "totp_recovery_code" (
    "user_id" UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    "code" TEXT NOT NULL,
    PRIMARY KEY ("code", "user_id")
);
