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
