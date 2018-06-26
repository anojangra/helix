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
	"krakenUSD",
];

pub static STRATEGIES: [&str; 12] = [
	"hhv:<ticker>:<param>",
	"llv:<ticker>:<param>",
	"conupdays:<ticker>:<param>",
	"condowndays:<ticker>:<param>",
	"gapup:<ticker>:<param>",
	"gapdown:<ticker>:<param>",
	"abovema:<ticker>:<param>",
	"belowma:<ticker>:<param>",
	"stdeva:<ticker>:<param>",
	"stdevb:<ticker>:<param>",
	"stdevd:<ticker>:<param>",
	"stdevf:<ticker>:<param>",
];

pub fn tickers_length() -> i32 {
	TICKERS.len() as i32
}

pub fn strategies_length() -> i32 {
	STRATEGIES.len() as i32
}

pub fn statements_length() -> i32 {
	STATEMENTS.len() as i32
}
