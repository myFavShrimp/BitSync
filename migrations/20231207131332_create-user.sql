CREATE TABLE "user" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "is_admin" BOOLEAN DEFAULT false NOT NULL
);
