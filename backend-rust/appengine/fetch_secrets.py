import sys
from google.cloud import storage

bucket_name = sys.argv[1]
blob_name = sys.argv[2]

# If you don't specify credentials when constructing the client, the
# client library will look for credentials in the environment.
client = storage.Client()

bucket = client.get_bucket(bucket_name)
blob = bucket.get_blob(blob_name)
print(blob.download_as_string().decode('utf-8').splitlines()[0])
