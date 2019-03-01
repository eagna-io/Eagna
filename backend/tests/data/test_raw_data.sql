INSERT INTO markets (id, title, organizer, short_desc, description, open_ts, close_ts, initial_coin_issue ) VALUES (
  1,
  'スマブラ大会の勝者は？',
  'Rohan Market .inc',
  '3/1に開催されるスマブラ大会の勝者を予想する',
  '3/1に開催されるスマブラ大会の勝者を予想する',
  1551358006,
  1552365206,
  10000
);

INSERT INTO market_tokens (name, market_id, initial_price)
  VALUES ('Yuya', 1, 10);
INSERT INTO market_tokens (name, market_id, initial_price)
  VALUES ('Atsuki', 1, 10);
INSERT INTO market_tokens (name, market_id, initial_price)
  VALUES ('Kohei', 1, 10);
