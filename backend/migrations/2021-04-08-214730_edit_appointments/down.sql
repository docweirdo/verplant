-- This file should undo anything in `up.sql`
ALTER TABLE "appointments" 
ADD COLUMN "date" TEXT NOT NULL DEFAULT "";