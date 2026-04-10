CREATE TYPE "session_platform" AS ENUM ('macos', 'windows', 'linux', 'ios', 'android', 'unknown');
CREATE TYPE "session_browser" AS ENUM ('chrome', 'firefox', 'safari', 'edge', 'opera', 'unknown');

CREATE TABLE "session" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "user_id" UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    "platform" session_platform NOT NULL,
    "browser" session_browser NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT now() NOT NULL,
    "last_seen_at" TIMESTAMPTZ DEFAULT now() NOT NULL
);
