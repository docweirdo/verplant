-- Your SQL goes here
CREATE TABLE "conducts" (
    "provider_id" INTEGER NOT NULL,
    "course_id" INTEGER NOT NULL,
    PRIMARY KEY ("provider_id", "course_id")
)