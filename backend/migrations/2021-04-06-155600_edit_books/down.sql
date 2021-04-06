-- This file should undo anything in `up.sql`
DROP TABLE "books";

CREATE TABLE IF NOT EXISTS "books"(
    "id" INTEGER PRIMARY KEY NOT NULL,
    "url" TEXT NOT NULL,
    "course_id" INTEGER NOT NULL,
    FOREIGN KEY ("course_id")
       REFERENCES "courses" ("id")
);