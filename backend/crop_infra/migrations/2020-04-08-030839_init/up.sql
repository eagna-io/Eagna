CREATE TABLE admins (
  id      UUID PRIMARY KEY,
  email   TEXT UNIQUE NOT NULL,
  cred    BYTEA NOT NULL, -- 64byte
  salt    BYTEA NOT NULL -- 64byte
);

CREATE TABLE accounts (
  id        UUID PRIMARY KEY,
  name      TEXT NOT NULL
);

CREATE TYPE contest_status AS ENUM (
  'upcoming',
  'open',
  'closed',
  'archived'
);

CREATE TABLE contests (
  id              UUID PRIMARY KEY,
  status          contest_status NOT NULL DEFAULT 'upcoming',
  title           TEXT NOT NULL,
  category        TEXT NOT NULL,
  /* 対応するイベントがいつ開催されるか */
  /* 「未定」のこともある */
  event_start_at  TIMESTAMPTZ
);

CREATE TYPE poll_status AS ENUM (
  'open',
  'closed'
);

CREATE TABLE polls (
  id                    UUID PRIMARY KEY,
  status                poll_status NOT NULL DEFAULT 'open',
  contest_id            UUID NOT NULL,
  title                 TEXT NOT NULL,
  duration_sec          INTEGER,
  idx                   INTEGER NOT NULL,
  created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
  resolved_at           TIMESTAMPTZ DEFAULT NULL,
  resolved_choice_name  TEXT DEFAULT NULL,

  UNIQUE (contest_id, title),
  CONSTRAINT contest_poll_fkey FOREIGN KEY (contest_id)
    REFERENCES contests (id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE choices (
  /* unused */
  id        SERIAL PRIMARY KEY,
  poll_id   UUID NOT NULL,
  name      TEXT NOT NULL,
  color     TEXT NOT NULL,
  idx       INTEGER NOT NULL,

  UNIQUE (poll_id, name),
  CONSTRAINT poll_choice_fkey FOREIGN KEY (poll_id)
    REFERENCES polls (id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE account_choices (
  /* unused */
  id          SERIAL PRIMARY KEY,
  account_id  UUID NOT NULL,
  poll_id     UUID NOT NULL,
  choice_name TEXT NOT NULL,

  UNIQUE (account_id, poll_id),
  CONSTRAINT account_choices_account_fkey FOREIGN KEY (account_id)
    REFERENCES accounts (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT account_choices_poll_fkey FOREIGN KEY (poll_id)
    REFERENCES polls (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT account_choices_choice_fkey FOREIGN KEY (poll_id, choice_name)
    REFERENCES choices (poll_id, name) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE comments (
  id          UUID PRIMARY KEY,
  contest_id  UUID, /* Contestに紐づくCommentの場合 */
  poll_id     UUID, /* Pollに紐づくCommentの場合 */
  account_id  UUID NOT NULL,
  choice_name TEXT, /* そのコメントをしたときに選択していたchoice */
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  content     TEXT NOT NULL,

  CONSTRAINT comment_contest_fkey FOREIGN KEY (contest_id)
    REFERENCES contests (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT comment_poll_fkey FOREIGN KEY (poll_id)
    REFERENCES polls (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT comment_account_fkey FOREIGN KEY (account_id)
    REFERENCES accounts (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CONSTRAINT comment_choice_fkey FOREIGN KEY (poll_id, choice_name)
    REFERENCES choices (poll_id, name) ON UPDATE RESTRICT ON DELETE RESTRICT
);
