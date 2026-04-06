CREATE TABLE "invite_token" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "is_admin" BOOLEAN DEFAULT false NOT NULL
);
