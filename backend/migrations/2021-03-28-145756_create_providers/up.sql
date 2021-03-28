-- Your SQL goes here
CREATE TABLE "providers" (
    "person_id" INTEGER PRIMARY KEY NOT NULL,
    "password_hash" TEXT,
    FOREIGN KEY ("person_id")
       REFERENCES "persons" ("id") 
)