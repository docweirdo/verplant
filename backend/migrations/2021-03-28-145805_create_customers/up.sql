-- Your SQL goes here
CREATE TABLE "customers" (
    "person_id" INTEGER PRIMARY KEY NOT NULL,
    "organisation" TEXT NOT NULL,
    "class" TEXT,
    FOREIGN KEY ("person_id")
       REFERENCES "persons" (id) 
)