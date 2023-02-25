-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP created_at;
ALTER TABLE "users" DROP updated_at;
DROP FUNCTION set_update_time();
DROP TRIGGER trig_update_user ON "users";
