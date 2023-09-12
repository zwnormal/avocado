use crate::db::SessionStore;
use crate::session::{Session, SessionId, UserId};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_query::{ColumnDef, Expr, Iden, Index, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Iden)]
enum SessionTable {
    #[iden = "session"]
    Table,
    Id,
    AccessToken,
    AccessTokenExpireAt,
    RefreshToken,
    RefreshTokenExpireAt,
    LoginAt,
    UserId,
    Email,
    FirstName,
    LastName,
    Role,
}

#[derive(sqlx::FromRow, Debug)]
struct SessionSqlite {
    id: Uuid,
    access_token: String,
    access_token_expire_at: DateTime<Utc>,
    refresh_token: String,
    refresh_token_expire_at: DateTime<Utc>,
    login_at: DateTime<Utc>,
    user_id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
    role: String,
}

impl From<SessionSqlite> for Session {
    fn from(value: SessionSqlite) -> Self {
        Session {
            id: value.id,
            access_token: value.access_token,
            access_token_expire_at: value.access_token_expire_at,
            refresh_token: value.refresh_token,
            refresh_token_expire_at: value.refresh_token_expire_at,
            login_at: value.login_at,
            user_id: value.user_id.into(),
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            role: value.role,
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
        let sql = Table::create()
            .table(SessionTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SessionTable::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(SessionTable::AccessToken).string())
            .col(ColumnDef::new(SessionTable::AccessTokenExpireAt).date_time())
            .col(ColumnDef::new(SessionTable::RefreshToken).string())
            .col(ColumnDef::new(SessionTable::RefreshTokenExpireAt).date_time())
            .col(ColumnDef::new(SessionTable::LoginAt).date_time())
            .col(ColumnDef::new(SessionTable::UserId).uuid())
            .col(ColumnDef::new(SessionTable::Email).string())
            .col(ColumnDef::new(SessionTable::FirstName).string())
            .col(ColumnDef::new(SessionTable::LastName).string())
            .col(ColumnDef::new(SessionTable::Role).string())
            .build(SqliteQueryBuilder);
        sqlx::query(&sql).execute(&pool).await.unwrap();
        Index::create()
            .name("idx-session-user-id")
            .table(SessionTable::Table)
            .col(SessionTable::UserId);
        Store { pool }
    }

    fn all_columns() -> Vec<SessionTable> {
        vec![
            SessionTable::Id,
            SessionTable::AccessToken,
            SessionTable::AccessTokenExpireAt,
            SessionTable::RefreshToken,
            SessionTable::RefreshTokenExpireAt,
            SessionTable::LoginAt,
            SessionTable::UserId,
            SessionTable::Email,
            SessionTable::FirstName,
            SessionTable::LastName,
            SessionTable::Role,
        ]
    }
}

#[async_trait]
impl SessionStore for Store {
    async fn insert(&self, session: Session) -> Result<SessionId> {
        let (sql, values) = Query::insert()
            .into_table(SessionTable::Table)
            .columns(Self::all_columns())
            .values([
                session.id.into(),
                session.access_token.into(),
                session.access_token_expire_at.into(),
                session.refresh_token.into(),
                session.refresh_token_expire_at.into(),
                session.login_at.into(),
                Uuid::from(session.user_id).into(),
                session.email.into(),
                session.first_name.into(),
                session.last_name.into(),
                session.role.into(),
            ])?
            .build_sqlx(SqliteQueryBuilder);
        sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(session.id)
    }

    async fn update(&self, session: Session) -> Result<()> {
        let (sql, values) = Query::update()
            .table(SessionTable::Table)
            .values([
                (SessionTable::AccessToken, session.access_token.into()),
                (
                    SessionTable::AccessTokenExpireAt,
                    session.access_token_expire_at.into(),
                ),
                (SessionTable::RefreshToken, session.refresh_token.into()),
                (
                    SessionTable::RefreshTokenExpireAt,
                    session.refresh_token_expire_at.into(),
                ),
                (SessionTable::LoginAt, session.login_at.into()),
                (SessionTable::UserId, Uuid::from(session.user_id).into()),
                (SessionTable::Email, session.email.into()),
                (SessionTable::FirstName, session.first_name.into()),
                (SessionTable::LastName, session.last_name.into()),
                (SessionTable::Role, session.role.into()),
            ])
            .and_where(Expr::col(SessionTable::Id).eq(session.id))
            .build_sqlx(SqliteQueryBuilder);
        sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(())
    }

    async fn get(&self, session_id: &SessionId) -> Result<Option<Session>> {
        let (sql, values) = Query::select()
            .columns(Self::all_columns())
            .from(SessionTable::Table)
            .and_where(Expr::col(SessionTable::Id).eq(*session_id))
            .limit(1)
            .build_sqlx(SqliteQueryBuilder);
        let mut rows = sqlx::query_as_with::<_, SessionSqlite, _>(&sql, values)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.pop().map(|u| u.into()))
    }

    async fn delete(&self, session_id: &SessionId) -> Result<()> {
        let (sql, values) = Query::delete()
            .from_table(SessionTable::Table)
            .and_where(Expr::col(SessionTable::Id).eq(*session_id))
            .build_sqlx(SqliteQueryBuilder);
        sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(())
    }

    async fn logout(&self, user_id: &UserId) -> Result<()> {
        let (sql, values) = Query::delete()
            .from_table(SessionTable::Table)
            .and_where(Expr::col(SessionTable::UserId).eq(Uuid::from(*user_id)))
            .build_sqlx(SqliteQueryBuilder);
        sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(())
    }
}
