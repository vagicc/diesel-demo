use crate::get_env;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{pg::PgConnection, Connection};

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn pg_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = get_env("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("无法创建Postgres连接池");
    pool
}

pub fn establish_connection() -> PgPooledConnection {
    let database_url = get_env("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("diesel::r2d2::Pool连接Postgres数据库出错");
    let conn = pool.get().expect("数据库连接出错");
    conn
}
