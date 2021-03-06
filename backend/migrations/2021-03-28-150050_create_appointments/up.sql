-- Your SQL goes here
CREATE TABLE "appointments" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "date" TEXT NOT NULL, -- DD-MM-YYYY
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
)