# rust-ddd-practice

映画館のチケット予約が題材

## 機能

- 映画の座席の予約
- キャンセル

- レディースデー
- シニア割

## シチュエーション

劇場端末とオンラインで予約できる。映画館は一つしかないものとし、映画館のサーバーに各クライアントがアクセスしている状況。

## dev

```
rustup default nightly
```

## layered architecture

router -> usecase -> service -> repository(domain) -> store

```ts
class Service {
  private _repo: Repository;

  constructor(repo: Repository) {
    this._repo = repo;
  }

  getUser(id: number) {
    this._repo.getUser(id);
  }
}
```

```ts
class Usecase {
  private _service_: Service;
  private _repo: Repository;

  constructor(service_: Service) {
    const service = new Service(this._repo);
    this._service_ = service_;
  }

  getUser(id: number) {
    this._service_.getUser(id);
  }
}
```
