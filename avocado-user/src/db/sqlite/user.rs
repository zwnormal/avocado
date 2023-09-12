use crate::db::UserStore;
use crate::domain::user::{Role, User, UserId};
use anyhow::Result;
use avocado_base::secret::SecretString;
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::{FirstName, LastName};
use fake::{Dummy, Fake, Faker};
use sea_query::{ColumnDef, Expr, Iden, Order, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Sqlite};
use ulid::Ulid;
use uuid::Uuid;

#[derive(Iden)]
enum UserTable {
    #[iden = "user"]
    Table,
    Id,
    Email,
    FirstName,
    LastName,
    PasswordHash,
    Role,
}

#[derive(sqlx::FromRow, Debug, Dummy)]
struct UserSqlite {
    id: Uuid,
    #[dummy(faker = "FreeEmail()")]
    email: String,
    #[dummy(faker = "FirstName()")]
    first_name: String,
    #[dummy(faker = "LastName()")]
    last_name: String,
    password_hash: String,
    #[dummy(faker = "0..=1")]
    role: i32,
}

impl From<UserSqlite> for User {
    fn from(value: UserSqlite) -> Self {
        User {
            id: value.id.into(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            password_hash: value.password_hash,
            role: value.role.try_into().unwrap_or(Role::NormalUser),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Store {
    pool: Pool<Sqlite>,
}

impl Store {
    pub(crate) async fn new() -> Self {
        let pool = sqlx::pool::PoolOptions::new()
            .max_lifetime(None)
            .idle_timeout(None)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Create the user table
        let sql = Table::create()
            .table(UserTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserTable::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(UserTable::FirstName).string())
            .col(ColumnDef::new(UserTable::LastName).string())
            .col(ColumnDef::new(UserTable::Email).string().unique_key())
            .col(ColumnDef::new(UserTable::PasswordHash).string())
            .col(ColumnDef::new(UserTable::Role).integer())
            .build(SqliteQueryBuilder);
        sqlx::query(&sql).execute(&pool).await.unwrap();
        let store = Store { pool };

        // Insert an admin user
        let admin = User {
            id: Ulid::new(),
            first_name: "System".to_string(),
            last_name: "Admin".to_string(),
            email: "admin@avocado.com".to_string(),
            password_hash: User::hash_password(SecretString::new("kIxv4NomLT0WwGKF".to_string()))
                .expect("unable to generate admin password hash"),
            role: Role::Admin,
        };
        store
            .insert(admin)
            .await
            .expect("unable to create admin user");

        if std::env::var("FAKE_DATA").is_ok() {
            for _ in 0..50 {
                let fake_user: UserSqlite = Faker.fake();
                store
                    .insert(fake_user.into())
                    .await
                    .expect("unable to create fake user");
            }
        }

        store
    }

    fn all_columns() -> Vec<UserTable> {
        vec![
            UserTable::Id,
            UserTable::FirstName,
            UserTable::LastName,
            UserTable::Email,
            UserTable::PasswordHash,
            UserTable::Role,
        ]
    }
}

#[tonic::async_trait]
impl UserStore for Store {
    async fn insert(&self, user: User) -> Result<UserId> {
        let (sql, values) = Query::insert()
            .into_table(UserTable::Table)
            .columns(Self::all_columns())
            .values([
                Uuid::from(user.id).into(),
                user.first_name.into(),
                user.last_name.into(),
                user.email.into(),
                user.password_hash.into(),
                (user.role as i32).into(),
            ])?
            .build_sqlx(SqliteQueryBuilder);
        sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(user.id)
    }

    async fn get(&self, user_id: &UserId) -> Result<Option<User>> {
        let (sql, values) = Query::select()
            .columns(Self::all_columns())
            .from(UserTable::Table)
            .and_where(Expr::col(UserTable::Id).eq(Uuid::from(*user_id)))
            .limit(1)
            .build_sqlx(SqliteQueryBuilder);
        let mut rows = sqlx::query_as_with::<_, UserSqlite, _>(&sql, values)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.pop().map(|u| u.into()))
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let (sql, values) = Query::select()
            .columns(Self::all_columns())
            .from(UserTable::Table)
            .and_where(Expr::col(UserTable::Email).eq(email))
            .limit(1)
            .build_sqlx(SqliteQueryBuilder);
        let mut rows = sqlx::query_as_with::<_, UserSqlite, _>(&sql, values)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.pop().map(|u| u.into()))
    }

    async fn list(&self) -> Result<Vec<User>> {
        let (sql, values) = Query::select()
            .columns(Self::all_columns())
            .from(UserTable::Table)
            .order_by(UserTable::Email, Order::Asc)
            .build_sqlx(SqliteQueryBuilder);
        let rows = sqlx::query_as_with::<_, UserSqlite, _>(&sql, values.clone())
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(|d| d.into()).collect())
    }

    async fn delete(&self, user_id: &UserId) -> Result<()> {
        let (sql, values) = Query::delete()
            .from_table(UserTable::Table)
            .and_where(Expr::col(UserTable::Id).eq(Uuid::from(*user_id)))
            .build_sqlx(SqliteQueryBuilder);
        sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(())
    }

    async fn update(&self, user_id: &UserId, user: User) -> Result<()> {
        let (sql, values) = Query::update()
            .table(UserTable::Table)
            .values([
                (UserTable::FirstName, user.first_name.into()),
                (UserTable::LastName, user.last_name.into()),
                (UserTable::Email, user.email.into()),
                (UserTable::PasswordHash, user.password_hash.into()),
            ])
            .and_where(Expr::col(UserTable::Id).eq(Uuid::from(*user_id)))
            .build_sqlx(SqliteQueryBuilder);
        sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::sqlite::user::Store;
    use crate::db::UserStore;
    use crate::domain::user::{Role, User};
    use ulid::Ulid;

    #[tokio::test]
    async fn test_user_store() {
        let user_db = Store::new().await;
        let first_user = User {
            id: Ulid::new(),
            first_name: "Wei".to_string(),
            last_name: "Zheng".to_string(),
            email: "william@test.com".to_string(),
            password_hash: "hash".to_string(),
            role: Role::Admin,
        };
        let first_user_id = user_db.insert(first_user.clone()).await.unwrap();

        let existing_user = user_db.get(&first_user_id).await.unwrap().unwrap();
        let non_existing_user = user_db.get(&Ulid::new()).await.unwrap();
        assert_eq!(existing_user, first_user);
        assert!(non_existing_user.is_none());

        let existing_user = user_db
            .get_by_email("william@test.com")
            .await
            .unwrap()
            .unwrap();
        let non_existing_user = user_db.get_by_email("tester@test.com").await.unwrap();
        assert_eq!(existing_user, first_user);
        assert!(non_existing_user.is_none());

        let mut second_user = User {
            id: Ulid::new(),
            first_name: "Robert".to_string(),
            last_name: "Li".to_string(),
            email: "robert@test.com".to_string(),
            password_hash: "hash".to_string(),
            role: Role::Admin,
        };
        let second_user_id = user_db.insert(second_user.clone()).await.unwrap();
        let existing_users = user_db.list().await.unwrap();
        assert_eq!(existing_users.len(), 3);
        assert_eq!(*existing_users.get(1).unwrap(), second_user);
        assert_eq!(*existing_users.get(2).unwrap(), first_user);

        user_db.delete(&first_user.id).await.unwrap();
        let existing_users = user_db.list().await.unwrap();
        assert_eq!(existing_users.len(), 2);
        assert_eq!(*existing_users.get(1).unwrap(), second_user);

        second_user.email = "robert.li@gmail.com".to_string();
        user_db.update(&second_user_id, second_user).await.unwrap();
        let existing_user = user_db.get(&second_user_id).await.unwrap().unwrap();
        assert_eq!(existing_user.email, "robert.li@gmail.com")
    }
}
