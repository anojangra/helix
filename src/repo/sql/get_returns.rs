pub fn sql() -> &'static str {
  " SELECT
      EXTRACT(epoch FROM ts) AS ts
    , (close::REAL - LAG(close::REAL, 1) OVER (ORDER BY ts)) / LAG(close::REAL, 1) OVER (ORDER BY ts) AS ret
    FROM btcc.hourly
    WHERE ticker = $1
    AND ts >= '2016-01-01'
    ORDER BY ts;"
}
