import json
import os

from lib.api.server import Server


def main():
  # サーバーの設定
  # 環境変数が指定されていない場合はエラー
  bind_host = os.environ['HOST']
  bind_port = int(os.environ['PORT'])

  # DBの設定
  db_url = os.environ.get('DB_URL')
  if db_url == None:
    db_url = getDBUrlFromCloudStorage()
  print(f"DB_URL : {db_url}")

  # API server の起動
  print(f"Server start on {bind_host}:{bind_port}")
  server = Server(bind_host, bind_port, db_url)
  server.serve_forever()


def getDBUrlFromCloudStorage():
  BUCKET_ID = 'server_secrets'

  from google.cloud import storage
  client = storage.Client()
  bucket = client.get_bucket(BUCKET_ID)
  blob = bucket.get_blob('db_url.txt')
  return blob.download_as_string().decode('utf-8').splitlines()[0]
  

main()
