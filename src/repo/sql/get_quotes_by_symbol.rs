pub fn sql() -> &'static str {
    "SELECT
       ticker,
       EXTRACT(epoch FROM ts),
       open::REAL,
       high::REAL,
       low::REAL,
       close::REAL,
       volume::REAL
    FROM btcc.hourly 
    WHERE ticker = $1 
    AND ts >= '2016-01-01'
    ORDER BY ts;"
}
