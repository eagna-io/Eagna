CREATE TABLE users (
  name        text UNIQUE NOT NULL,
  hashed_pass text NOT NULL,
  coins       integer DEFAULT 0
);

CREATE TABLE access_tokens (
  token       text UNIQUE NOT NULL,
  user_name   text NOT NULL,
  is_valid    boolean NOT NULL,
  created_at  integer NOT NULL,
  CONSTRAINT access_token_user_fkey FOREIGN KEY(user_name)
    REFERENCES users(name) ON UPDATE CASCADE ON DELETE RESTRICT
);
