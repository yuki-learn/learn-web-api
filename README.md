# Web API The Good Parts まとめ
本のまとめ: https://github.com/yuki-learn/learn-web-api/tree/main/book

## TODOアプリ
本の内容を意識して簡単なTODOのWebAPIを作成。


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
* postgresql