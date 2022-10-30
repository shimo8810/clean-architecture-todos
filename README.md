# Clean Architecture ToDos API

Clean Architecture にしたがってTODO-APIを実装してみた.

- `domain`: ドメイン層, エンティティや値オブジェクト,リポジトリのトレイトが実装されている.
- `usecase`: ユースケース層, アプリケーションサービスやDTOなどが実装されている.
- `infra`: インフラ層(DB層), 基本的にはDB(diesel + Postgresql)が実装されている.
- `server`: サーバー層(controller層): 基本的にはサーバー(actix-web)が実装されている.

## Setup
```
diesel migration run
cargo run
```

## CRUD
### Create
```bash
$ curl -X POST localhost:8080/tasks -H "Content-Type: application/json" -d '{"body": "花に水をあげる"}'
{"id":"5e4e30cf-5e9c-4611-bb5e-400cfed55a2a","body":"花に水をあげる","state":false}⏎
```
### Read
```bash
$ curl -X GET localhost:8080/tasks
[{"id":"5e4e30cf-5e9c-4611-bb5e-400cfed55a2a","body":"花に水をあげる","state":false},{"id":"71c57ade-dcd5-4f5b-b84e-f1119a7e1e25","body":"コーヒーを淹れる","state":false}]
```
### Update
```bash
$ curl -X PUT localhost:8080/tasks -H "Content-Type: application/json" -d '{"id": "5e4e30cf-5e9c-4611-bb5e-400cfed55a2a", "body": "花に水をあげる", "state": true}'
update
```
### Remove
```bash
curl -X DELETE localhost:8080/tasks/5e4e30cf-5e9c-4611-bb5e-400cfed55a2a
delete id: 5e4e30cf-5e9c-4611-bb5e-400cfed55a2a
```