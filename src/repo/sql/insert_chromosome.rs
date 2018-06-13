/// Insert chromosome sql
/// 
/// id: $1
/// ticker: $2
/// chromosome: $3
/// dna: $4
/// generation: $5
/// chromosome_length: $6
pub fn sql() -> &'static str {
    "INSERT INTO trade_chromosomes
		(id, ticker, chromosome, dna, generation, chromosome_length)
	 VALUES ($1, $2, $3, $4, $5, $6);"
}