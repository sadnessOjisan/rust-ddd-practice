use async_trait::async_trait;
use domain::User;
use mockall::automock;
use repository::UserRepository;
#[automock]
#[async_trait]
pub trait UserService {
    // fn new() -> Self;
    // ここに self ないと継承先で呼べない
    async fn get_user_by_id(&self) -> User;
}

pub struct UserServiceImpl<T>
where
    T: UserRepository + Send + Sync,
{
    pub user_repository: T,
}

#[async_trait]
impl<T: UserRepository + Send + Sync> UserService for UserServiceImpl<T> {
    async fn get_user_by_id(&self) -> User {
        let dto = self.user_repository.get_user_by_id();
        let user = User {
            id: dto.id,
            age: dto.age,
            gender: dto.gender,
        };
        self.user_repository.getUsers().await;
        user
    }
}
