use domain::User;
use repository::{UserRepository, UserRepositoryComponent};

pub trait UserService: UserRepositoryComponent {
    fn get_user_by_id(&self, id: i32) -> () {
        let repo = self.user_dao();
        let data = repo.getUserById(id);
    }
}

pub struct UserServiceImpl<Repo: UserRepository> {
    repo: Repo,
}

impl<T: UserRepositoryComponent> UserService for T {}

pub trait UserServiceComponent {
    type UserService: UserService;
    fn user_service(&self) -> Self::UserService;
}

#[cfg(test)]
mod tests {
    use repository::UserRepository;

    use crate::UserServiceImpl;

    #[test]
    fn test_get_user_by_id() {
        struct UserMockRepository {}

        impl UserRepository for UserMockRepository {
            fn getUserById(&self, id: i32) -> () {
                ();
            }
        }

        let mock = UserMockRepository {};

        let service = UserServiceImpl { repo: mock };
        let res = service.repo.getUserById(1);
        assert_eq!((), res);
    }
}
