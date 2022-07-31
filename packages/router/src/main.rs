use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// trait UserRepository {
//     fn get_users(id: i32) -> () {}
// }

// struct UserPgDao();

// impl UserRepository for UserPgDao {
//   fn get_users(&self, id: i32) -> () {
//   }
// }

// struct UserService<U: UserRepository>(U);

// impl<U:UserRepository> UserService<U> {
//   pub fn get_user_by_id(&self, id: i32) -> () {
//     self.0.get_users(id)
//   }
// }

// * scala, なんで class を tarit でかぶせられるのか
// * scala でつくるときのインスタンスの作り方
// * keen さんの例で作る (cake, arc)

// Non-copyable types.
// コピー不可な型
// 訳注: `clone()`メソッドを用いないかぎり、値のコピーではなくムーブが起きる型

#[derive(Debug)]
struct Empty;
struct Null;

// A trait generic over `T`.
// ジェネリック型 `T`に対するトレイト
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    // `self`に加えてもう一つジェネリック型を受け取り、
    // 何もしないメソッドのシグネチャを定義
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
// `U`を`self`として、`T`をもう一つの引数として受け取る`DoubleDrop<T>`
// を実装する。`U`,`T`はいずれもジェネリック型
impl<T, U: Debug> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    // このメソッドは2つの引数の所有権を取り、メモリ上から開放する。
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`.
    // `empty`と`null`を開放
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
    // ^ TODO: これらの行をアンコメントしてみましょう。
}

use std::fmt::{Debug, Display};

struct Show<T: Display> {
    x: T,
}

// impl側にもトレイト境界が必要となる
impl<T: Display> Show<T> {
    fn print(&self) {
        println!("{:#}", self.x);
    }
}

// もしくは、型引数にはDisplayトレイトが実装された型のみ取ることが出来る
impl Show<String> {
    fn hoge(&self) {}
}

fn hoge() {
    let show = Show::<String> { x: "".to_string() };
    show.print();
    show.hoge();
}

// 上述記事中のUserRepository相当
pub trait UserDao {
    fn find_user(&self, id: i32) -> ();
}
// 上述記事中のUserRepositoryComponent相当
pub trait HaveUserDao {
    type UserDao: UserDao;
    fn user_dao(&self) -> Self::UserDao;
}

// 上述記事中のUserService相当
pub trait UserService: HaveUserDao {
    fn get_user_by_id(&self, id: i32) -> () {
        self.user_dao().find_user(id)
    }
}

// UserServiceはHaveUserDaoにのみ依存するのでそれさえ実装していれば自動で実装を与えられます。
// もちろんテストなどで挙動を上書きしたければ具体的な型での実装で上書きできます。
impl<T: HaveUserDao> UserService for T {}

struct RepositoryImpl<Repo: UserDao> {
    repo: Repo,
}

// 上述記事中のUserServiceComponent相当
trait HaveUserService {
    type UserService: UserService;
    fn user_service(&self) -> Self::UserService;
}

struct ServiceImpl<Service: UserService> {
    service: Service,
}

#[cfg(test)]
mod tests {
    use crate::{HaveUserDao, HaveUserService, ServiceImpl, UserDao, UserService};

    #[test]
    fn test_get_user_by_id() {
        struct MockRepository {}
        impl UserDao for MockRepository {
            fn find_user(&self, id: i32) -> () {
            }
        }
        struct DaoComponent {}
        impl HaveUserDao for DaoComponent {
            type UserDao = MockRepository;
            fn user_dao(&self) -> Self::UserDao {
                MockRepository {}
            }
        }

        let mock_repo = DaoComponent{};
        let service = ServiceImpl {
            service: mock_repo
        };
        let user = service.service.get_user_by_id(2);
        assert_eq!(user, 3);
    }
}
