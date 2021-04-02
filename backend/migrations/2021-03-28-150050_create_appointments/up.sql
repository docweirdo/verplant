-- Your SQL goes here
CREATE TABLE "appointments" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "date" TEXT NOT NULL, -- DD-MM-YYYY
    "start_time" TEXT NOT NULL, -- hh:mm
    "end_time" TEXT NOT NULL, -- hh:mm
    "books_id" INTEGER NOT NULL,
    "proposer_id" INTEGER NOT NULL, 
    "room_id" INTEGER NOT NULL, 
    "state" TEXT NOT NULL, -- allowed: "SUGGESTION", "PENDING", "APPROVED", "REJECTED"
    FOREIGN KEY ("books_id")
       REFERENCES "books" ("id"),
    FOREIGN KEY ("proposer_id")
       REFERENCES "persons" ("id"),
    FOREIGN KEY ("room_id")
       REFERENCES "rooms" ("id")
)