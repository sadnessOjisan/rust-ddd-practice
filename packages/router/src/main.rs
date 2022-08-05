use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use mockall::*;
use mockall::predicate::*;
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

// struct Repository;
// #[automock]
// impl Repository {
//     fn new() -> Repository {
//         Repository {}
//     }

//     fn get_users(self) -> () {}
// }

// struct Service {
//     repository: Repository,
// }
// #[automock]
// impl Service {
//     fn new(Repository: Repository) -> Service {
//         Service {
//             repository: Repository,
//         }
//     }
//     fn get_users(self) -> () {}
// }

// pub struct Usecase {
//     service: Service,
// }

// impl Usecase {
//     fn new(service: Service) -> Usecase {
//         Usecase { service: service }
//     }
//     fn get_users(self) -> () {
//         self.get_users()
//     }
// }

// async fn handler() -> Html<&'static str> {
//     let repo = Repository {};
//     let service = Service { repository: repo };
//     let usecase = Usecase { service: service };
//     let actual = usecase.get_users();
//     Html("<h1>Hello, World!</h1>")
// }

// async fn handler2() -> Html<&'static str> {
//     Html("<h1>Hello, World!</h1>")
// }

trait UserRepository {
    // ここに self ないと継承先で呼べない
    fn get_user_by_id(&self) -> ();
}

struct RepositoryImpl {}

impl UserRepository for RepositoryImpl {
    fn get_user_by_id(&self) -> () {
        
    }
}

#[automock]
trait UserService {
    // fn new() -> Self;
    // ここに self ないと継承先で呼べない
    fn get_user_by_id(&self) -> ();
}

struct UserServiceImpl<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T: UserRepository> UserService for UserServiceImpl<T> {
    fn get_user_by_id(&self) -> () {
        self.user_repository.get_user_by_id()
    }
}

#[automock]
trait UserUsecase {
    fn get_user_by_id(&self) -> ();
}

struct UserUsecaseImpl<T>
where
    T: UserService,
{
    user_service: T,
}

impl<T: UserService> UserUsecase for UserUsecaseImpl<T> {
    fn get_user_by_id(&self) -> () {
        self.user_service.get_user_by_id()
    }
}

async fn handler() -> Html<&'static str> {
    let repo = RepositoryImpl {};
    let service = UserServiceImpl { user_repository: repo };
    let usecase = UserUsecaseImpl { user_service: service };
    let actual = usecase.get_user_by_id();
    Html("<h1>Hello, World!</h1>")
}

#[cfg(test)]
mod tests {
    use crate::{UserUsecaseImpl, MockUserService, UserService, UserUsecase};

    #[test]
    fn test_usecase() {
        let mut service = MockUserService::new();
        service.expect_get_user_by_id().returning(|| ());
        let usecase = UserUsecaseImpl { user_service: service };
        let user = usecase.get_user_by_id();
        assert_eq!(user, ());
    }
}
