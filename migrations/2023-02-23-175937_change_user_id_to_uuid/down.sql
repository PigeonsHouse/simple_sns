-- This file should undo anything in `up.sql`
ALTER TABLE "users" ALTER COLUMN id TYPE varchar(64);
ALTER TABLE "users" ALTER COLUMN id DROP DEFAULT;