use std::env;

use diesel::{Connection, QueryDsl, QueryResult, RunQueryDsl};
use diesel::pg::PgConnection;
use dotenv::dotenv;

use crate::core::models::{BnsKline, BnsPair, NewBnsPair};
use crate::core::storage::bns_pairs::dsl::bns_pairs as q_bns_pairs;
use crate::schema::{bns_klines, bns_pairs};

pub struct Storage {
    pg: PgConnection,
}

impl Storage {
    pub fn create_bns_klines(self, klines: Vec<BnsKline>) -> QueryResult<usize> {
        let result = diesel::insert_into(bns_klines::table)
            .values(&klines)
            .on_conflict_do_nothing()
            .execute(&self.pg)?;

        Ok(result)
        // .get_results(&self.pg)
        // .expect("Error saving bns klines")

        // use crate::schema::bns_klines::dsl::bns_klines as dsl;
        // dsl.load::<Vec<BnsKline>>(&self.pg).expect("klines loaded")
    }
    pub fn create_bns_pair(self, pair: NewBnsPair) -> BnsPair {
        diesel::insert_into(bns_pairs::table)
            .values(&pair)
            .on_conflict_do_nothing()
            .execute(&self.pg)
            // .get_result(&self.pg)
            .expect("Error creating bns pair");

        use diesel::{ExpressionMethods};
        q_bns_pairs
            .filter(bns_pairs::ticker.eq(pair.ticker))
            .filter(bns_pairs::interval.eq(pair.interval))
            .first::<BnsPair>(&self.pg).unwrap()
    }

    pub fn get_bns_pair(self, ticker: String, interval: String) -> Result<BnsPair, String> {
        use diesel::{ExpressionMethods};
        let result = q_bns_pairs
            .filter(bns_pairs::ticker.eq(ticker))
            .filter(bns_pairs::interval.eq(interval))
            .first::<BnsPair>(&self.pg);

        match result {
            Ok(v) => Ok(v),
            Err(v) => Err(String::from(v.to_string()))
        }
    }
}

pub fn new() -> Storage {
    Storage { pg: establish_connection() }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
