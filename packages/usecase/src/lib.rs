use domain::User;
use service::{UserService, UserServiceComponent};
//

// trait UserService  {
//    fn getUser(&self, id: u16) -> User;
// }



// impl Usecase {
//     fn new() -> Usecase{}
// }

// impl UserUsecase for Usecase {}
// impl MoiveUsecae for Usecase {}

// trait QueryUsecase: UserService + MoiveService {}

// trait serviceProvider {
//     type UseCase: QueryUsecase;
//     fn provideUserUsecase() -> Self::Usecase;
// }

// Usecase (struct) -> 本体、全てのservice呼べる（Userに限らず）、全パターンの各ユースケースを実装する
// UserUsecase (trait) -> 各ユースケース。
// UserUsecase (trait)
// serviceProvider(trait) 
// Service ()

// fn main(){
//     let repo = Repository {}

//     let controller = Controller {
//         provier: serviceProvider,
//         repo: repo;
//     };

//     let service = Service {};

//     let usecase = Usecase {
//         service: &service
//     };

//     controller.get("/", usecase).get("/:id", userUsecase)
// }

pub trait UserUsecase: UserServiceComponent {
    fn get_user_by_id(&self, id: i32) -> () {
        let service = self.user_service();
        let user = service.get_user_by_id(id);
    }
}

impl<T: UserServiceComponent> UserUsecase for T {}

trait UserUsecaseComponent {
    type UserUsecase: UserUsecase;
    fn user_usecase(&self) -> Self::UserUsecase;
}