pub static STATEMENTS: [&str; 2] = ["<stmnt>", "<stmnt>::<code>"];

pub static TICKERS: [&str; 13] = [ 
    "coinbaseUSD",
	"zaifJPY",
	"bitstampUSD",
	"coincheckJPY",
	"btcnCNY",
	"bitflyerJPY",
	"btceUSD",
	"btctradeCNY",
	"coinbaseEUR",
	"bitfinexUSD",
	"fiscoJPY",
	"krakenEUR",
	"krakenUSD"
];


pub static STRATEGIES: [&str; 2] = [
	"hhv:<ticker>:<param>",
	"llv:<ticker>:<param>",
];

// pub static STRATEGIES: [&str; 6] = [
// 	"hhv:<ticker>:<param>",
// 	"llv:<ticker>:<param>",
// 	"conupdays:<ticker>:<param>",
// 	"condowndays:<ticker>:<param>",
// 	"gapup:<ticker>:<param>",
// 	"gapdown:<ticker>:<param>",
// ];

pub static TARGET_TICKER: &str = "krakenUSD";

// Mutate probably of dna applied after crossover
pub static MUTATE_PROB: f32 = 0.20;

// The number of fittest chromosomes to evolve
pub static FITTEST: usize = 500;

pub static POPULATION_SIZE: i32 = 10000;

pub fn tickers_length() -> i32 {
	TICKERS.len() as i32
}

pub fn strategies_length() -> i32 {
	STRATEGIES.len() as i32
}

pub fn statements_length() -> i32 {
	STATEMENTS.len() as i32
}
