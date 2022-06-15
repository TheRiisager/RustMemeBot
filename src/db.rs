use mysql::*;
use mysql::prelude::*;
use std::vec::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Meme {
    id: i32,
    pub link: String,
}

pub struct Database {
    conn: PooledConn
}

impl Database {
    pub fn new() -> Self {
        println!("making DBhandler");
        let opts = Opts::from_url("mysql://memebot:907413349e0f87c8ef8551ac4b9848ef@localhost:3306/memedb").unwrap();
        let pool = Pool::new(opts).unwrap();

        Self {
            conn: pool.get_conn().unwrap(),
        }
    }

    pub fn get_meme(&mut self) -> Result<Meme> {
        let result: Result<Vec<Row>> = self.conn.query(
            "SELECT * from memes
            ORDER BY RAND()
            LIMIT 1;"
        );

        match result {
            Ok(rows)=> {
                let row = &rows[0];
                let meme = Meme {
                    id: row.get(0).unwrap(),
                    link: row.get(1).unwrap()
                };
                return Ok(meme);
            }
            Err(error)=> {
                println!("Couldn't complete query: {:?}", error);
                return Err(error);
            }
        }
    }

    pub fn add_meme(&mut self, link: &str) -> std::result::Result<(), mysql::Error> {
        let mut tx = self.conn.start_transaction(TxOpts::default())?;
        let statement = tx.exec_drop("
            INSERT INTO memes (link)
            VALUES (?)
        ", (link,));
        tx.commit()?;
        return statement;
    }
}