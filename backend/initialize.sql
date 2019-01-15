CREATE TABLE users (
  id          serial primary key,
  name        text NOT NULL,
  hashed_pass text NOT NULL,
  hold_coin   integer DEFAULT 0
);
