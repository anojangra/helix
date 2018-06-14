use repo::pg;
use repo::sql::get_returns;
use schemas::Return;
use postgres;

/// Get returns
///
pub fn call(ticker: String) -> Vec<Return> {
    let conn = pg::connect();
    let rows = &conn.query(get_returns::sql(), &[&ticker]).unwrap();
    let mut target_returns: Vec<Return> = vec![];
    for row in rows {
        let ret: Option<Result<f32, postgres::Error>> = row.get_opt(1);
        let r =  match ret {
            None => panic!("No value"),
            Some(Ok(_v)) => Return { ts: row.get("ts"), ret: row.get("ret")},
            Some(Err(_msg)) => Return { ts: row.get("ts"), ret: 0.0},
        };
        target_returns.push(r);
    }
    target_returns
}

