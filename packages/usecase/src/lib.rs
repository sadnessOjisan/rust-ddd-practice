use service::UserService;
// 上述記事中のUserServiceComponent相当
pub trait UserServiceComponent {
    type UserService: UserService;
    fn user_service(&self) -> Self::UserService;
}

pub trait UserUsecase: UserServiceComponent {
    fn get_user_by_id(&self, id: i32) -> () {
        self.user_service().get_user_by_id(id)
    }
}

impl<T: UserServiceComponent> UserUsecase for T {}

struct UserUsecaseImpl<Usecase: UserUsecase> {
    usecase: Usecase,
}

#[cfg(test)]
mod tests {
    use service::{UserRepositoryComponent, UserService};

    use crate::{UserServiceComponent, UserUsecaseImpl};

    #[test]
    fn test_get_user_by_id() {
        struct MockService {}
        impl UserService for MockService {
        }
        impl UserRepositoryComponent for MockService {}
        struct ServiceComponent {}
        impl UserServiceComponent for ServiceComponent {
            type UserService = MockService;
            fn user_service(&self) -> Self::UserService {
                MockService {}
            }
        }

        let mock_service = ServiceComponent {};
        let usecase = UserUsecaseImpl { usecase: mock_service };
        let user = usecase.service.get_user_by_id(2);
        assert_eq!(user, ());
    }
}
