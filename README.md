
## 環境構築
[Taskfile](https://taskfile.dev/)のインストール
### Mac
```bash
brew install go-task
```

### docker
起動
```bash
docker-compose -f ./docker/compose.yml up --build
```

コンテナ内に入る雨酷いので、
```bash
docker-compose -f ./docker/compose.yml exec rust-app bash
```


### 参考資料
- [Rust公式ドキュメント](https://doc.rust-lang.org/book/title-page.html)
- [rust-analizer](https://rust-analyzer.github.io/manual.html#configuration)
