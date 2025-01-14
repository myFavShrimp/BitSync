CREATE TABLE "user" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "username" VARCHAR(255) NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "is_admin" BOOLEAN DEFAULT false NOT NULL,
    "totp_secret" BYTEA NOT NULL,
    "is_totp_set_up" BOOLEAN DEFAULT false NOT NULL
);
