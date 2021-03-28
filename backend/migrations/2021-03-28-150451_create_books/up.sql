-- Your SQL goes here
CREATE TABLE "books" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "url" TEXT NOT NULL,
    "course_id" INTEGER NOT NULL,
    FOREIGN KEY ("course_id")
       REFERENCES "courses" ("id") 
)