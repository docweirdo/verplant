-- Your SQL goes here
CREATE TABLE "courses" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "name" TEXT NOT NULL,
    "default_duration" INTEGER, -- in minutes
    "default_room_id" INTEGER,
    FOREIGN KEY ("default_room_id")
       REFERENCES "rooms" ("id")
)