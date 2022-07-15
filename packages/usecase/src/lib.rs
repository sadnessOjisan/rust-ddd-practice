use domain::User;

//

trait UserService  {
   fn getUser(&self, id: u16) -> User;
}



impl Usecase {
    fn new() -> Usecase{}
}

impl UserUsecase for Usecase {}
impl MoiveUsecae for Usecase {}

trait QueryUsecase: UserService + MoiveService {}

trait serviceProvider {
    type UseCase: QueryUsecase;
    fn provideUserUsecase() -> Self::Usecase;
}

// Usecase (struct) -> 本体、全てのservice呼べる（Userに限らず）、全パターンの各ユースケースを実装する
// UserUsecase (trait) -> 各ユースケース。
// UserUsecase (trait)
// serviceProvider(trait) 
// Service ()

fn main(){
    let repo = Repository {}

    let controller = Controller {
        provier: serviceProvider,
        repo: repo;
    };

    let service = Service {};

    let usecase = Usecase {
        service: &service
    };

    controller.get("/", usecase).get("/:id", userUsecase)
}