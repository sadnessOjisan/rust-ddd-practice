use async_trait::async_trait;
use domain::User;
use mockall::automock;
use service::UserService;

#[async_trait]
#[automock]
pub trait UserUsecase {
   async fn get_user_by_id(&self, id: i32) -> User;
}

pub struct UserUsecaseImpl<T>
where
    T: UserService + Send + Sync,
{
    pub user_service: T,
}

#[async_trait]
impl<T: UserService + Send + Sync> UserUsecase for UserUsecaseImpl<T> {
    async fn get_user_by_id(&self, id: i32) -> User {
        self.user_service.get_user_by_id().await
    }
}
