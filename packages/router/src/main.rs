use axum::{response::Html, routing::get, Router};
use repository::RepositoryImpl;
use service::UserServiceImpl;
use std::net::SocketAddr;
use usecase::{UserUsecaseImpl, UserUsecase};

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

async fn handler() -> String {
    let repo = RepositoryImpl {};
    let service = UserServiceImpl {
        user_repository: repo,
    };
    let usecase = UserUsecaseImpl {
        user_service: service,
    };
    let actual = usecase.get_user_by_id(1).await;
    format!("<h1>Hello, World! {}</h1>", actual.id)
}

#[cfg(test)]
mod tests {
    use domain::User;
    use service::MockUserService;
    use usecase::{UserUsecase, UserUsecaseImpl};

    #[tokio::test]
    async fn test_usecase() {
        let mut service = MockUserService::new();
        service.expect_get_user_by_id().returning(|| User {
            id: 2,
            age: 100,
            gender: domain::Gender::Male,
        });
        let usecase = UserUsecaseImpl {
            user_service: service,
        };
        let user = usecase.get_user_by_id(1).await;
        assert_eq!(
            user,
            User {
                id: 2,
                age: 100,
                gender: domain::Gender::Male
            }
        );
    }
}
