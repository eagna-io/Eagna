CREATE TABLE users (
  id            text PRIMARY KEY,
  name          text UNIQUE NOT NULL,
  email         text UNIQUE NOT NULL,
  is_admin      boolean NOT NULL DEFAULT False
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
  lmsr_b              integer NOT NULL,
  open_time           timestamptz NOT NULL,
  close_time          timestamptz NOT NULL,
  status              market_status NOT NULL DEFAULT 'preparing',
  settle_token_id     integer DEFAULT NULL, /* MUST NULL if "status" is NOT 'settled' */

  /* If "status" is 'settled', then "settle_token_id" MUST NOT NULL  */
  CONSTRAINT if_status_is_settled_then_settle_token_id_is_not_null
    CHECK ( (NOT status = 'settled') OR (settle_token_id IS NOT NULL) ),

  /* If "status" is NOT 'settled', then "settle_token_id" MUST NULL  */
  CONSTRAINT if_status_is_not_settled_then_settle_token_id_is_null
    CHECK ( (status = 'settled') OR (settle_token_id IS NULL) )
);

CREATE TABLE market_tokens (
  id                  serial PRIMARY KEY,
  name                text NOT NULL,
  description         text NOT NULL,
  market_id           integer NOT NULL,

  CONSTRAINT market_tokens_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TYPE order_type AS ENUM (
 'normal',
 'initial_supply',
 'settle'
);

CREATE TABLE orders (
  /* Required by diesel. But not used by program */
  id            serial PRIMARY KEY,
  market_id     integer NOT NULL,
  /* An serial number used to obtain optimistic lock */
  market_internal_serial_num  integer NOT NULL,
  user_id       text NOT NULL,
  /* MUST NULL if "type" is 'initial_supply' */
  token_id      integer,
  amount_token  integer NOT NULL,
  amount_coin   integer NOT NULL,
  type          order_type NOT NULL DEFAULT 'normal',
  time          timestamptz NOT NULL DEFAULT now(),

  UNIQUE (market_id, market_internal_serial_num),
  CONSTRAINT order_user_fkey FOREIGN KEY(user_id)
    REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT order_market_fkey FOREIGN KEY(market_id)
    REFERENCES markets(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  CONSTRAINT order_token_fkey FOREIGN KEY(token_id)
    REFERENCES market_tokens(id) ON UPDATE CASCADE ON DELETE RESTRICT,

  /* If "type" is 'initial_supply', then "token_id" MUST NULL  */
  CONSTRAINT if_type_is_initial_supply_then_token_id_is_null
    CHECK ( (NOT type = 'initial_supply') OR (token_id IS NULL) ),

  /* If "type" is NOT 'initial_supply', then "token_id" MUST NOT NULL  */
  CONSTRAINT if_type_is_not_initial_supply_then_token_id_is_not_null
    CHECK ( (type = 'initial_supply') OR (token_id IS NOT NULL) )
);

CREATE INDEX ON orders (market_id);
