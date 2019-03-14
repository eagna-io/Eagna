CREATE TABLE users (
  id          serial PRIMARY KEY,
  name        text UNIQUE NOT NULL,
  email       text UNIQUE NOT NULL,
  hashed_pass text NOT NULL
);

CREATE TABLE access_tokens (
  token         text UNIQUE NOT NULL,
  user_id       integer NOT NULL,
  force_expired boolean NOT NULL DEFAULT False,
  created_at    timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT access_token_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TYPE market_status AS ENUM (
  'preparing',
  'open',
  'closed',
  'settled'
);

CREATE TABLE markets (
  id                  serial PRIMARY KEY,
  title               text NOT NULL,
  organizer           text NOT NULL,
  short_desc          text NOT NULL,
  description         text NOT NULL,
  open_time           timestamptz NOT NULL,
  close_time          timestamptz NOT NULL,
  initial_coin_issue  integer NOT NULL,
  status              market_status NOT NULL DEFAULT 'preparing'
);

CREATE TABLE market_tokens (
  id                  serial PRIMARY KEY,
  name                text NOT NULL,
  description         text NOT NULL,
  market_id           integer NOT NULL,
  initial_price       integer NOT NULL,
  is_settlement_token boolean,
  CONSTRAINT market_tokens_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TYPE order_type AS ENUM (
 'normal',
 'initial_supply',
 'reword'
);

CREATE TABLE orders (
  id            serial PRIMARY KEY,
  user_id       integer NOT NULL,
  market_id     integer NOT NULL,
  token_id      integer,
  amount_token  integer NOT NULL,
  amount_coin   integer NOT NULL,
  type          order_type NOT NULL DEFAULT 'normal',
  time          timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT order_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT order_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT order_token_fkey FOREIGN KEY(token_id)
    REFERENCES market_tokens(id) ON UPDATE CASCADE ON DELETE RESTRICT
);
