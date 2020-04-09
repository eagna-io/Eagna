CREATE TABLE accounts (
  id        UUID PRIMARY KEY,
  name      TEXT NOT NULL
);

CREATE TABLE contests (
  id  UUID PRIMARY KEY
);

CREATE TABLE polls (
  id                  UUID PRIMARY KEY,
  contest_id          UUID NOT NULL,
  title               TEXT NOT NULL,
  created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
  end_at              TIMESTAMPTZ NOT NULL,
  resolved_choice_id  TEXT DEFAULT NULL,

  UNIQUE (contest_id, title),
  CONSTRAINT contest_poll_fkey FOREIGN KEY(contest_id)
    REFERENCES contests(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE choices (
  /* unused */
  id        SERIAL PRIMARY KEY,
  poll_id   UUID NOT NULL,
  name      TEXT NOT NULL,
  color     TEXT NOT NULL,
  idx       INTEGER NOT NULL DEFAULT 0,

  UNIQUE (poll_id, name),
  CONSTRAINT poll_choice_fkey FOREIGN KEY(poll_id)
    REFERENCES polls(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE account_choices (
  /* unused */
  id          SERIAL PRIMARY KEY,
  account_id  UUID NOT NULL,
  poll_id     UUID NOT NULL,
  choice_name TEXT NOT NULL,

  UNIQUE (account_id, poll_id),
  CONSTRAINT account_choices_account_fkey FOREIGN KEY(account_id)
    REFERENCES accounts(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT account_choices_poll_fkey FOREIGN KEY(poll_id)
    REFERENCES polls(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE comments (
  id          SERIAL PRIMARY KEY,
  poll_id     UUID NOT NULL,
  account_id  UUID NOT NULL,
  content     TEXT NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),

  CONSTRAINT poll_comment_fkey FOREIGN KEY(poll_id)
    REFERENCES polls(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT account_comment_fkey FOREIGN KEY(account_id)
    REFERENCES accounts(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);
