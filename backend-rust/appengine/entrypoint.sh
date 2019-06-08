#!/bin/bash

BUCKET_NAME=server_secrets

PG_URL=$(python3 fetch_secrets.py $BUCKET_NAME pg_url.txt)
REDIS_URL=$(python3 fetch_secrets.py $BUCKET_NAME redis_url.txt)
FIREBASE_API_KEY=$(python3 fetch_secrets.py $BUCKET_NAME firebase_api_key.txt)

PG_URL=$PG_URL REDIS_URL=$REDIS_URL FIREBASE_API_KEY=$FIREBASE_API_KEY ./eagna
