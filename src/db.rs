use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

use tower_web::extract::{Context, Extract, Immediate};
use tower_web::util::BufStream;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn connect() -> MysqlPool {
    init(dotenv!("DB_URL")).expect("Error")
}

pub struct ApiParam {
    pub connection: MysqlPool,
}

impl<B: BufStream> Extract<B> for ApiParam {
    type Future = Immediate<ApiParam>;

    fn extract(context: &Context) -> Self::Future {
        let conn = connect();

        let param = ApiParam { connection: conn };
        Immediate::ok(param)
    }
}
