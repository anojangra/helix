pub fn sql() -> &'static str {
  " SELECT * FROM
    (
    SELECT
      EXTRACT(epoch FROM ts) AS ts
    , (LEAD(close::real, 1) OVER (ORDER BY ts) - close::real) / close::real AS ret
    FROM btcc.hourly
    WHERE ticker = $1
    AND ts >= '2016-01-01'
    ) a
    WHERE ret IS NOT NULL
    ORDER BY ts;
  "
}
