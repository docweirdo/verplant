-- This file should undo anything in `up.sql`
-- Here you can drop column
CREATE TABLE IF NOT EXISTS "new_table"( 
   "person_id" INTEGER PRIMARY KEY NOT NULL,
    "password_hash" TEXT,
    FOREIGN KEY ("person_id")
    REFERENCES "persons" ("id") );

-- copy data from the table to the new_table
INSERT INTO "new_table"
SELECT "person_id", "password_hash"
FROM "providers";

-- drop the table
DROP TABLE "providers";

-- rename the new_table to the table
ALTER TABLE "new_table" RENAME TO "providers"; 
