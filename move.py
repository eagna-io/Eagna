import psycopg2
import uuid

old_url = ''
new_url = ''

def main():
    move_users()
    move_markets()
    move_market_tokens()
    move_orders()


################
## Move users ##
################
def move_users():
    for (uid, name, email, is_admin) in query_users():
        print(name)
        insert_user(uid, name, email, is_admin)

def query_users():
    with psycopg2.connect(old_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT id, name, email, is_admin FROM users')
            return cur.fetchall()

def insert_user(uid, name, email, is_admin):
    with psycopg2.connect(new_url) as conn:
        with conn.cursor() as cur:
            cur.execute('INSERT INTO users (fb_uid, name, email, is_admin) VALUES (%s, %s, %s, %s)', (uid, name, email, is_admin))


##################
## Move markets ##
##################
def move_markets():
    for (title, desc, lmsr_b, open, close, status, settle_token_id) in query_markets():
        resolved_token_name = None if settle_token_id == None else query_token_name(settle_token_id)
        converted_status = convert_status(status)
        print(title)
        insert_market(title, desc, lmsr_b, open, close, converted_status, resolved_token_name)

def query_markets():
    with psycopg2.connect(old_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT title, description, lmsr_b, open_time, close_time, status, settle_token_id FROM markets')
            return cur.fetchall()

def query_token_name(token_id):
    with psycopg2.connect(old_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT name FROM market_tokens WHERE id = %s', (token_id,))
            return cur.fetchone()[0]

def convert_status(old_status):
    if old_status == 'preparing':
        return 'upcoming'
    elif old_status == 'settled':
        return 'resolved'
    else:
        return old_status

def insert_market(title, desc, lmsr_b, open, close, status, resolved_token_name):
    id = str(uuid.uuid4())
    with psycopg2.connect(new_url) as conn:
        with conn.cursor() as cur:
            cur.execute('INSERT INTO markets (id, title, organizer_id, description, lmsr_b, open, close, status, resolved_token_name) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s)', (id, title, 'e643a0da-dc5c-4c2d-9585-c2c6da0cf77d', desc, lmsr_b, open, close, status, resolved_token_name))


########################
## Move market_tokens ##
########################
def move_market_tokens():
    for (name, desc, market_title) in query_market_tokens():
        market_id = query_market_id(market_title)
        insert_market_tokens(name, desc, market_id)

def query_market_tokens():
    with psycopg2.connect(old_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT market_tokens.name, market_tokens.description, markets.title FROM market_tokens INNER JOIN markets ON market_tokens.market_id = markets.id')
            return cur.fetchall()

def query_market_id(title):
    with psycopg2.connect(new_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT id FROM markets WHERE title = %s', (title,))
            return cur.fetchone()[0]

def insert_market_tokens(name, desc, market_id):
    with psycopg2.connect(new_url) as conn:
        with conn.cursor() as cur:
            cur.execute('INSERT INTO market_tokens (name, description, sumbnail_url, market_id) VALUES (%s, %s, %s, %s)', (name, desc, '', market_id))


#################
## Move orders ##
#################
def move_orders():
    for (market_title, local_id, user_id, token_id, amount_token, amount_coin, type, time) in query_orders():
        if amount_coin == 0 and type == 'settle':
            continue
        else:
            market_id = query_market_id(market_title)
            token_name = None if token_id == None else query_token_name(token_id)
            new_type = convert_type(type)
            print(market_id, token_name, new_type)
            insert_order(market_id, local_id, user_id, token_name, amount_token, amount_coin, new_type, time)

def query_orders():
    with psycopg2.connect(old_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT markets.title, market_internal_serial_num, user_id, token_id, amount_token, amount_coin, type, time FROM orders INNER JOIN markets ON markets.id = orders.market_id')
            return cur.fetchall()

def query_market_id(title):
    with psycopg2.connect(new_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT id FROM markets WHERE title = %s', (title,))
            return cur.fetchone()[0]

def query_token_name(token_id):
    with psycopg2.connect(old_url) as conn:
        with conn.cursor() as cur:
            cur.execute('SELECT name FROM market_tokens WHERE id = %s', (token_id,))
            return cur.fetchone()[0]

def convert_type(old_type):
    if old_type == 'initial_supply':
        return 'coin_supply'
    elif old_type == 'settle':
        return 'reward'
    else:
        return 'normal'

def insert_order(market_id, local_id, user_id, token_name, amount_token, amount_coin, type_, time):
    with psycopg2.connect(new_url) as conn:
        with conn.cursor() as cur:
            cur.execute('INSERT INTO orders (market_id, market_local_id, user_id, token_name, amount_token, amount_coin, type, time) VALUES (%s, %s, %s, %s, %s, %s, %s, %s)', (market_id, local_id, user_id, token_name, amount_token, amount_coin, type_, time))

main()
