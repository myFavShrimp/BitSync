CREATE TABLE "user_share" (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "item_path" TEXT NOT NULL
);
