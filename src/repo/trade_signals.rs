use repo::pg;

pub fn copy(filename: &String) {
    let conn = pg::connect();
    let sql = format!("COPY trade_signals FROM '{}';", filename);
    conn.execute(&sql, &[])
        .unwrap();
}


// COPY trade_signals FR/OM dcf5855d-f991-4ffd-8540-c77b48759640.txt'