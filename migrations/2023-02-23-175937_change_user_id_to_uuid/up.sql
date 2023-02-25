-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
ALTER TABLE "users" ALTER COLUMN id TYPE UUID USING (uuid_generate_v4());
ALTER TABLE "users" ALTER COLUMN id SET DEFAULT uuid_generate_v4();