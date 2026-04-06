CREATE TYPE "session_platform" AS ENUM ('macos', 'windows', 'linux', 'ios', 'unknown');

CREATE TABLE "session" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "user_id" UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    "platform" session_platform NOT NULL DEFAULT 'unknown',
    "created_at" TIMESTAMPTZ DEFAULT now() NOT NULL,
    "last_seen_at" TIMESTAMPTZ DEFAULT now() NOT NULL
);
