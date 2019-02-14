use actix::prelude::*;
use diesel::prelude::*;
use failure::Error;
use transaction::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use transaction::diesel::DieselContext;
use transaction::diesel::with_conn;


pub fn count() -> impl Transaction<Ctx=DieselContext<PgConnection>, Item=u64, Err=Error> + Send + 'static {
    use schema::categories::dsl;
    with_conn(move |cn| {
        dsl::categories
            .count()
            .get_result(cn)
            .map(|count: i64| count as u64)
            .map_err(Into::into)
    })
}


#[derive(Clone)]
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl DbExecutor {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        DbExecutor(pool)
    }
    fn run<Tx>(&self, tx: Tx) -> Result<Tx::Item, Error>
    where
        Tx: Transaction<Ctx = DieselContext<PgConnection>, Err = Error> + Send + 'static,
        Tx::Ctx: Send + 'static,
        Tx::Item: Send +'static,
        Tx::Err: Send + 'static,
    {
        let conn: Result<_, Error> = self
            .0
            .get()
            .map_err(|err| err.into());
        let conn: &PgConnection = &*conn.unwrap();
        transaction::diesel::run(conn, tx)
            .map_err(|err| err.into())
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct TxMessage<Tx>(pub Tx);

impl<Tx> Message for TxMessage<Tx>
where
    Tx: Transaction<Ctx = DieselContext<PgConnection>, Err = Error> + Send + 'static,
    Tx::Ctx: Send + 'static,
    Tx::Item: Send +'static,
    Tx::Err: Send + 'static,
{
    type Result = Result<Tx::Item, Error>;
}

impl<Tx> Handler<TxMessage<Tx>> for DbExecutor
where
    Tx: Transaction<Ctx = DieselContext<PgConnection>, Err = Error> + Send + 'static,
    Tx::Ctx: Send + 'static,
    Tx::Item: Send + 'static,
    Tx::Err: Send + 'static,
{
    type Result = Result<Tx::Item, Error>;

    fn handle(&mut self, tx: TxMessage<Tx>, _ctx: &mut Self::Context) -> Self::Result {
        self.run(tx.0)
    }
}
