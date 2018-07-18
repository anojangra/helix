/// sql for getting quotes by symbol
pub fn get_quotes_by_symbol() -> &'static str {
  "SELECT
       ticker,
       EXTRACT(epoch FROM ts),
       open::REAL,
       high::REAL,
       low::REAL,
       close::REAL,
       volume::REAL
    FROM av_quotes 
    WHERE ticker = $1 
    AND ts >= '2000-01-01'
    ORDER BY ts;"
}

/// sql for getting returns
pub fn get_returns() -> &'static str {
  " SELECT * FROM
    (
    SELECT
      EXTRACT(epoch FROM ts) AS ts
    , (LEAD(close::real, 1) OVER (ORDER BY ts) - close::real) / close::real AS ret
    FROM av_quotes
    WHERE ticker = $1
    AND ts >= '2000-01-01'
    ) a
    WHERE ret IS NOT NULL
    ORDER BY ts;
  "
}

/// sql for getting tickers
pub fn get_tickers() -> &'static str {
  "SELECT ticker
    FROM av_quotes
    GROUP BY ticker;"
}

/// Insert chromosome sql
/// 
/// ## Params
/// 
/// ```
/// id: $1
/// ticker: $2
/// chromosome: $3
/// dna: $4
/// generation: $5
/// chromosome_length: $6
/// ```
pub fn insert_chromosome() -> &'static str {
  "INSERT INTO trade_chromosomes
		(id, ticker, chromosome, dna, generation, chromosome_length)
	 VALUES ($1, $2, $3, $4, $5, $6);"
}
