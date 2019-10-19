-- 過去のmarketのreward_pointsを0にするためにDEFAULT 0を設定
ALTER TABLE markets ADD COLUMN total_reward_point INTEGER NOT NULL DEFAULT 0;
-- DEFAULT値を削除
ALTER TABLE markets ALTER COLUMN total_reward_point DROP DEFAULT;

-- Market報酬として発行されたpoint報酬の履歴
CREATE TABLE market_reward_records (
  -- アプリ的に使用することはないが、dieselのために必要
  unused_id   serial PRIMARY KEY,
  market_id   uuid NOT NULL,
  user_id     text NOT NULL,
  -- 発行したポイント量。0より大きい。0の場合はレコードを追加しない。
  point       integer NOT NULL,
  time        timestamptz NOT NULL DEFAULT now(),

  CONSTRAINT user_reward_point_history_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT user_reward_point_history_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(fb_uid) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT point_larger_than_zero CHECK ( point > 0 )
);

COMMENT ON TABLE market_reward_records IS
  'Market報酬として発行されたpoint報酬の履歴';
COMMENT ON COLUMN market_reward_records.unused_id IS
  'アプリ的に使用することはないが、dieselのために必要';
COMMENT ON COLUMN market_reward_records.point IS
  '発行したポイント量。0より大きい。0の場合はレコードを追加しない';

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

  CONSTRAINT price_larger_than_zero CHECK ( point > 0 )
);

COMMENT ON TABLE prizes IS
  'Userがpointと交換可能な景品';
COMMENT ON COLUMN prizes.point IS
  'prizeを交換するのに必要なポイント量。0より大きい';


CREATE TYPE prize_trade_status as ENUM (
  'requested',
  'processed'
);

CREATE TABLE user_prize_trade_history (
  -- アプリ的に使用することはないが、dieselのために必要
  unused_id     serial PRIMARY KEY,
  user_id       text NOT NULL,
  prize_id      uuid NOT NULL,
  -- 消費したポイント。0より大きい。
  point         integer NOT NULL,
  time          timestamptz NOT NULL DEFAULT now(),
  status        prize_trade_status NOT NULL DEFAULT 'requested',

  CONSTRAINT user_prize_trade_history_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(fb_uid) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT user_prize_trade_history_prize_fkey FOREIGN KEY(prize_id)
    REFERENCES prizes(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT price_larger_than_zero CHECK ( point > 0 )
);
