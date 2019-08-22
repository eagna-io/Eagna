CREATE TABLE users (
  /* Firebase uid */
  fb_uid        text PRIMARY KEY,
  name          text NOT NULL,
  email         text UNIQUE NOT NULL,
  is_admin      boolean NOT NULL DEFAULT False,
  created       timestamptz NOT NULL DEFAULT now()
);

CREATE TYPE market_status AS ENUM (
  'upcoming',
  'open',
  'closed',
  'resolved'
);

CREATE TABLE organizers (
  id            uuid PRIMARY KEY,
  name          text NOT NULL,
  sumbnail_url  text NOT NULL
);

CREATE TABLE markets (
  id                  uuid PRIMARY KEY,
  title               text NOT NULL,
  organizer_id        uuid NOT NULL,
  description         text NOT NULL,
  lmsr_b              integer NOT NULL,
  open                timestamptz NOT NULL,
  close               timestamptz NOT NULL,
  status              market_status NOT NULL DEFAULT 'upcoming',
  /* MUST NULL if "status" is NOT 'resolved' */
  resolved_token_name text DEFAULT NULL,

  CONSTRAINT market_organizer_fkey FOREIGN KEY(organizer_id)
    REFERENCES organizers(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TABLE market_tokens (
  /* Required by diesel. But not used by program */
  unused_id     serial PRIMARY KEY,
  /* MUST be locally unique in market */
  name          text NOT NULL,
  description   text NOT NULL,
  sumbnail_url  text NOT NULL,
  market_id     uuid NOT NULL,

  UNIQUE (market_id, name),
  CONSTRAINT market_tokens_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE INDEX ON market_tokens (market_id);

CREATE TABLE market_prizes (
  /* Required by diesel. But not used by program */
  unused_id       serial PRIMARY KEY,
  /* A locally unique number in each market */
  market_local_id integer NOT NULL,
  name            text NOT NULL,
  sumbnail_url    text NOT NULL,
  target          text NOT NULL,
  market_id       uuid NOT NULL,

  UNIQUE (market_id, market_local_id),
  CONSTRAINT market_prizes_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TYPE order_type AS ENUM (
 'normal',
 'coin_supply',
 'reward'
);

CREATE TABLE orders (
  /* Required by diesel. But not used by program */
  unused            serial PRIMARY KEY,
  /* A locally unique number in each market */
  market_local_id   integer NOT NULL,
  user_id           text NOT NULL,
  /* MUST NULL if "type" is 'initial_supply' */
  token_name        text,
  amount_token      integer NOT NULL,
  amount_coin       integer NOT NULL,
  type              order_type NOT NULL DEFAULT 'normal',
  time              timestamptz NOT NULL DEFAULT now(),
  market_id         uuid NOT NULL,

  UNIQUE (market_id, market_local_id),
  CONSTRAINT order_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(fb_uid) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT order_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE INDEX ON orders (market_id);
