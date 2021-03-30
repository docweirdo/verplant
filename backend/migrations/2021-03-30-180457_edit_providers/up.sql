-- Your SQL goes here
ALTER TABLE "providers" 
ADD COLUMN "is_admin" INTEGER NOT NULL DEFAULT 0;