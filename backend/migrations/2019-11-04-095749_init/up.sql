CREATE TABLE users (
  id            UUID PRIMARY KEY,
  name          TEXT NOT NULL,
  email         TEXT UNIQUE NOT NULL,
  is_admin      BOOLEAN NOT NULL DEFAULT False,
  created       TIMESTAMPTZ NOT NULL DEFAULT now(),
  credential    BYTEA NOT NULL, -- 64byte
  salt          BYTEA NOT NULL -- 64byte
);

CREATE TYPE market_status AS ENUM (
  'upcoming',
  'open',
  'closed',
  'resolved'
);

CREATE TABLE organizers (
  id            UUID PRIMARY KEY,
  name          TEXT NOT NULL,
  thumbnail_url TEXT NOT NULL
);

CREATE TABLE markets (
  id                  UUID PRIMARY KEY,
  title               TEXT NOT NULL,
  organizer_id        UUID NOT NULL,
  description         TEXT NOT NULL,
  lmsr_b              INTEGER NOT NULL,
  open                TIMESTAMPTZ NOT NULL,
  close               TIMESTAMPTZ NOT NULL,
  status              market_status NOT NULL DEFAULT 'upcoming',
  /* MUST NULL if "status" is NOT 'resolved' */
  resolved_token_name TEXT DEFAULT NULL,
  total_reward_point  INTEGER NOT NULL,
  resolved_at         TIMESTAMPTZ DEFAULT NULL,

  CONSTRAINT market_organizer_fkey FOREIGN KEY(organizer_id)
    REFERENCES organizers(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE market_tokens (
  /* Required by diesel. But not used by program */
  unused_id     SERIAL PRIMARY KEY,
  /* MUST be locally unique in market */
  name          TEXT NOT NULL,
  description   TEXT NOT NULL,
  thumbnail_url TEXT NOT NULL,
  market_id     UUID NOT NULL,
  idx           INTEGER NOT NULL DEFAULT 0,

  UNIQUE (market_id, name),
  CONSTRAINT market_tokens_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE INDEX ON market_tokens (market_id);

CREATE TABLE market_prizes (
  /* Required by diesel. But not used by program */
  unused_id       SERIAL PRIMARY KEY,
  /* A locally unique number in each market */
  market_local_id INTEGER NOT NULL,
  name            TEXT NOT NULL,
  thumbnail_url   TEXT NOT NULL,
  target          TEXT NOT NULL,
  market_id       UUID NOT NULL,

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
  unused            SERIAL PRIMARY KEY,
  /* A locally unique number in each market */
  market_local_id   INTEGER NOT NULL,
  user_id           UUID NOT NULL,
  /* MUST NULL if "type" is 'initial_supply' */
  token_name        TEXT,
  amount_token      INTEGER NOT NULL,
  amount_coin       INTEGER NOT NULL,
  type              order_type NOT NULL DEFAULT 'normal',
  time              TIMESTAMPTZ NOT NULL DEFAULT now(),
  market_id         UUID NOT NULL,

  UNIQUE (market_id, market_local_id),
  CONSTRAINT order_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT order_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE INDEX ON orders (market_id);

-- Market報酬として発行されたpoint報酬の履歴
CREATE TABLE market_reward_records (
  -- アプリ的に使用することはないが、dieselのために必要
  unused_id   SERIAL PRIMARY KEY,
  market_id   UUID NOT NULL,
  user_id     UUID NOT NULL,
  -- 発行したポイント量。0より大きい。0の場合はレコードを追加しない。
  point       INTEGER NOT NULL,

  CONSTRAINT user_reward_point_history_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT user_reward_point_history_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT point_larger_than_zero CHECK ( point > 0 )
);

-- Userがpointと交換可能な景品
CREATE TABLE prizes (
  id            UUID NOT NULL PRIMARY KEY,
  name          TEXT NOT NULL,
  description   TEXT NOT NULL,
  thumbnail_url TEXT NOT NULL,
  -- prizeを交換するのに必要なポイント量。0より大きい。
  point         INTEGER NOT NULL,
  -- prizeが交換可能かどうか。
  available     BOOLEAN NOT NULL DEFAULT true,
  created       TIMESTAMPTZ NOT NULL DEFAULT now(),

  CONSTRAINT point_larger_than_zero CHECK ( point > 0 )
);

CREATE TYPE prize_trade_status as ENUM (
  'requested',
  'processed'
);

CREATE TABLE user_prize_trade_records (
  id            UUID PRIMARY KEY,
  user_id       UUID NOT NULL,
  prize_id      UUID NOT NULL,
  -- 消費したポイント。0より大きい。
  point         INTEGER NOT NULL,
  time          TIMESTAMPTZ NOT NULL DEFAULT now(),
  status        prize_trade_status NOT NULL DEFAULT 'requested',
  processed_at  TIMESTAMPTZ DEFAULT NULL,

  CONSTRAINT user_prize_trade_history_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT user_prize_trade_history_prize_fkey FOREIGN KEY(prize_id)
    REFERENCES prizes(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT price_larger_than_zero CHECK ( point > 0 )
);
