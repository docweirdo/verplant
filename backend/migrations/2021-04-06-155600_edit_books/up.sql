-- Your SQL goes here
DROP TABLE "books";

CREATE TABLE IF NOT EXISTS "books"(
    "id" INTEGER PRIMARY KEY NOT NULL,
    "url" TEXT NOT NULL,
    "course_id" INTEGER NOT NULL,
    "customer_id" INTEGER NOT NULL,
    FOREIGN KEY ("course_id")
       REFERENCES "courses" ("id"), 
    FOREIGN KEY ("customer_id")
       REFERENCES "customers" ("person_id")
);

