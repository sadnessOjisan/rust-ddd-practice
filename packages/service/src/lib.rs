use domain::User;
struct UserService {
    //  repo
}

trait UserService  {
    fn getUser(&self, id: u16) -> User;
 }

impl UserService {
    fn getUser(id: u16) -> User {
        User {
            id: 1,
            age: 18,
            gender: domain::Gender::Female
        }
    }
}
