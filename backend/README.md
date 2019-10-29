## Tests

- Firebase infra はモックを使う
- FIREBASE_API_KEY を USE_MOCK_FIREBASE に設定するとモックを使用するようになる

### 全てのテストを実行

`make test`

### 特定のテストを実行

`tests/run.sh tests/scenarios/scenario_01.py`

### Verbose モードで実行

`TEST_VERBOSE=1 make test`

### テストの追加

/tests/scenarios/*.py ファイルを作成する
