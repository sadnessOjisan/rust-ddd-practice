use axum::Extension;
use repository::RepositoryImpl;
use service::UserServiceImpl;
use std::{net::SocketAddr, sync::Arc};
use usecase::{UserUsecase, UserUsecaseImpl};

pub struct Registry {
    // trait を食わせるとNG
    user_usecase: UserUsecaseImpl<UserServiceImpl<RepositoryImpl>>,
}

impl Registry {
    fn new() -> Self {
        let repo = RepositoryImpl {};
        let service = UserServiceImpl {
            user_repository: repo,
        };
        let usecase = UserUsecaseImpl {
            user_service: service,
        };
        Self {
            user_usecase: usecase,
        }
    }
}

#[tokio::main]
async fn main() {
    use axum::{routing::get, Extension, Router};
    use std::sync::Arc;
    let state = Arc::new(Registry::new());

    let app = Router::new()
        .route("/", get(handler))
        // Add middleware that inserts the state into all incoming request's
        // extensions.
        .layer(Extension(state));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(state: Extension<Arc<Registry>>) -> String {
    let usecase = &state.0.user_usecase;
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
