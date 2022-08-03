use repository::UserRepository;

// 上述記事中のUserRepositoryComponent相当
pub trait UserRepositoryComponent {
    type UserRepo: UserRepository;
    fn user_repository(&self) -> Self::UserRepo;
}

pub trait UserService: UserRepositoryComponent {
    fn get_user_by_id(&self, id: i32) -> () {
        self.user_repository().find_user(id)
    }
}

impl<T: UserRepositoryComponent> UserService for T {}

struct UserServiceImpl<Service: UserService> {
    service: Service,
}

#[cfg(test)]
mod tests {
    use crate::{UserRepository, UserRepositoryComponent, UserService, UserServiceImpl};

    #[test]
    fn test_get_user_by_id() {
        struct MockRepository {}
        impl UserRepository for MockRepository {
            fn find_user(&self, id: i32) -> () {}
        }
        struct DaoComponent {}
        impl UserRepositoryComponent for DaoComponent {
            type UserRepo = MockRepository;
            fn user_repository(&self) -> Self::UserRepo {
                MockRepository {}
            }
        }

        let mock_repo = DaoComponent {};
        let service = UserServiceImpl { service: mock_repo };
        let user = service.service.get_user_by_id(2);
        assert_eq!(user, ());
    }
}
