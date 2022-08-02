use domain::TheaterData;
use futures_util::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;
// struct Repository {
//     data: TheaterData,
// }

// impl Repository {
//     pub async fn getUsers() -> Result<(), sqlx::Error> {
//         let pool = MySqlPoolOptions::new()
//             .max_connections(5)
//             .connect("mysql://root:root_password@localhost:3306/rust_ddd_practice")
//             .await?;

//         // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
//         let mut rows = sqlx::query("SELECT * from user").fetch(&pool);

//         while let Some(row) = rows.try_next().await? {
//             // map the row into a user-defined domain type
//             dbg!(row);
//         }

//         Ok(())
//     }
// }

pub trait UserRepository {
    fn getUserById(&self, id: i32) -> ();
}

struct UserDbRepository {}

impl UserRepository for UserDbRepository {
    fn getUserById(&self, id: i32) -> () {
        // ...
    }
}

pub trait UserRepositoryComponent {
    type UserRepository: UserRepository;
    fn user_dao(&self) -> Self::UserRepository;
}
