use async_trait::async_trait;
use domain::Gender;
use futures_util::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;
pub struct UserDTO {
    pub id: u32,
    pub age: u8,
    pub gender: Gender,
}

#[async_trait]
pub trait UserRepository {
    // ここに self ないと継承先で呼べない
    fn get_user_by_id(&self) -> UserDTO;
    async fn getUsers(&self) -> Result<Vec<UserDTO>, sqlx::Error>;
}

pub struct RepositoryImpl {}

#[async_trait]
impl UserRepository for RepositoryImpl {
    fn get_user_by_id(&self) -> UserDTO {
        UserDTO {
            id: 100,
            age: 1,
            gender: Gender::Female,
        }
    }

    async fn getUsers(&self) -> Result<Vec<UserDTO>, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:root_password@localhost:3306/rust_ddd_practice")
            .await?;

        // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
        let mut rows = sqlx::query("SELECT * from user").fetch(&pool);

        while let Some(row) = rows.try_next().await? {
            // map the row into a user-defined domain type
            dbg!(row);
        }

        Ok(vec![])
    }
}
