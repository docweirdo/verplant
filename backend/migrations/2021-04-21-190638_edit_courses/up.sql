-- Your SQL goes here

CREATE TABLE IF NOT EXISTS "new_table"( 
   "id" INTEGER PRIMARY KEY NOT NULL,
    "name" TEXT NOT NULL,
    "default_duration" INTEGER, -- in minutes
    "default_room_id" INTEGER NOT NULL,
    "course_type" TEXT,
    FOREIGN KEY ("default_room_id")
       REFERENCES "rooms" ("id")
);


-- copy data from the table to the new_table
INSERT INTO new_table
SELECT "id", "name", "default_duration", "default_room_id", "course_type"
FROM "courses";

-- drop the table
DROP TABLE "courses";

-- rename the new_table to the table
ALTER TABLE "new_table" RENAME TO "courses"; 

