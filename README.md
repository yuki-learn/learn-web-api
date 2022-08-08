# Web API The Good Parts まとめ
本のまとめ: https://github.com/yuki-learn/learn-web-api/tree/main/book

## TODOアプリ
本の内容を意識して簡単なTODOのWebAPIを作成。

ソースコード: https://github.com/yuki-learn/learn-web-api/tree/main/todo-app/server

### URI
ベースURI: `/api/v1`

* todoを一覧取得: `GET /api/v1/todos`
* idでtodoを取得: `GET /api/v1/todos/:id`
* todoの新規登録: `POST /api/v1/todos`
* todoの更新: `PUT /api/v1/todos/:id`
* todoの削除: `DELETE /api/v1/todos/:id`

## 環境
* rustc 1.62.0
* actix-web
* diesel
* postgresql: [こちら](https://hub.docker.com/layers/postgres/library/postgres/12/images/sha256-15017a063c249afe1a87f6c6b163eddc3205601040b22c0f8b10625ae6c75402?context=explore)のDocker imageをそのままローカルで起動