use sqlx::PgPool;

#[derive(Clone)]
pub struct AppData {
    pub pool: PgPool,
}
