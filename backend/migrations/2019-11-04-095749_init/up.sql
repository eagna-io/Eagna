CREATE TABLE users (
  /* Firebase uid */
  fb_uid        text PRIMARY KEY,
  name          text NOT NULL,
  email         text UNIQUE NOT NULL,
  is_admin      boolean NOT NULL DEFAULT False,
  created       timestamptz NOT NULL DEFAULT now(),
  credential    VARCHAR(32) NOT NULL,
  salt          VARCHAR(32) NOT NULL
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
  thumbnail_url  text NOT NULL
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
  total_reward_point  integer NOT NULL,
  resolved_at         timestamptz DEFAULT NULL,

  CONSTRAINT market_organizer_fkey FOREIGN KEY(organizer_id)
    REFERENCES organizers(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE market_tokens (
  /* Required by diesel. But not used by program */
  unused_id     serial PRIMARY KEY,
  /* MUST be locally unique in market */
  name          text NOT NULL,
  description   text NOT NULL,
  thumbnail_url  text NOT NULL,
  market_id     uuid NOT NULL,
  idx           integer NOT NULL DEFAULT 0,

  UNIQUE (market_id, name),
  CONSTRAINT market_tokens_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE INDEX ON market_tokens (market_id);

CREATE TABLE market_prizes (
  /* Required by diesel. But not used by program */
  unused_id       serial PRIMARY KEY,
  /* A locally unique number in each market */
  market_local_id integer NOT NULL,
  name            text NOT NULL,
  thumbnail_url    text NOT NULL,
  target          text NOT NULL,
  market_id       uuid NOT NULL,

  UNIQUE (market_id, market_local_id),
  CONSTRAINT market_prizes_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE RESTRICT ON DELETE RESTRICT
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
    REFERENCES users(fb_uid) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT order_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE INDEX ON orders (market_id);

-- Market報酬として発行されたpoint報酬の履歴
CREATE TABLE market_reward_records (
  -- アプリ的に使用することはないが、dieselのために必要
  unused_id   serial PRIMARY KEY,
  market_id   uuid NOT NULL,
  user_id     text NOT NULL,
  -- 発行したポイント量。0より大きい。0の場合はレコードを追加しない。
  point       integer NOT NULL,

  CONSTRAINT user_reward_point_history_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT user_reward_point_history_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(fb_uid) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT point_larger_than_zero CHECK ( point > 0 )
);

-- Userがpointと交換可能な景品
CREATE TABLE prizes (
  id            uuid NOT NULL PRIMARY KEY,
  name          text NOT NULL,
  description   text NOT NULL,
  thumbnail_url text NOT NULL,
  -- prizeを交換するのに必要なポイント量。0より大きい。
  point         integer NOT NULL,
  -- prizeが交換可能かどうか。
  available     boolean NOT NULL DEFAULT true,
  created       timestamptz NOT NULL DEFAULT now(),

  CONSTRAINT point_larger_than_zero CHECK ( point > 0 )
);

CREATE TYPE prize_trade_status as ENUM (
  'requested',
  'processed'
);

CREATE TABLE user_prize_trade_records (
  id            uuid PRIMARY KEY,
  user_id       text NOT NULL,
  prize_id      uuid NOT NULL,
  -- 消費したポイント。0より大きい。
  point         integer NOT NULL,
  time          timestamptz NOT NULL DEFAULT now(),
  status        prize_trade_status NOT NULL DEFAULT 'requested',
  processed_at  timestamptz DEFAULT NULL,

  CONSTRAINT user_prize_trade_history_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(fb_uid) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT user_prize_trade_history_prize_fkey FOREIGN KEY(prize_id)
    REFERENCES prizes(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT price_larger_than_zero CHECK ( point > 0 )
);
