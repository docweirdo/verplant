-- Your SQL goes here
CREATE TABLE "new_table" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "start_time" TEXT NOT NULL, -- ISO8601
    "end_time" TEXT NOT NULL, -- ISO8601
    "books_id" INTEGER NOT NULL,
    "proposer_id" INTEGER NOT NULL, 
    "room_id" INTEGER NOT NULL, 
    "state" TEXT NOT NULL, -- allowed: "SUGGESTED", "APPROVED", "REJECTED"
    FOREIGN KEY ("books_id")
       REFERENCES "books" ("id"),
    FOREIGN KEY ("proposer_id")
       REFERENCES "persons" ("id"),
    FOREIGN KEY ("room_id")
       REFERENCES "rooms" ("id")
);

-- copy data from the table to the new_table
INSERT INTO "new_table"
SELECT "id", "start_time", "end_time", "books_id", "proposer_id", "room_id", "state"
FROM "appointments";

-- drop the table
DROP TABLE "appointments";

-- rename the new_table to the table
ALTER TABLE "new_table" RENAME TO "appointments"; 
