
## 環境構築

### docker
起動
```bash
docker-compose -f ./docker/compose.yml up --build
```

コンテナ内に入る
```bash
docker-compose -f ./docker/compose.yml exec rust-app bash
```
