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

// 上述記事中のUserRepository相当
pub trait UserRepository {
    fn find_user(&self, id: i32) -> ();
}

struct UserRepositoryImpl<Repo: UserRepository> {
    repo: Repo,
}
