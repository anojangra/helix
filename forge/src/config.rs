pub static STATEMENTS: [&str; 2] = ["<stmnt>", "<stmnt>::<code>"];

pub static TICKERS: [&str; 505] = [
	"MMM", "AOS", "ABT", "ABBV", "ACN", "ATVI", "AYI", "ADBE", "AAP", "AMD", "AES", "AET", "AMG",
	"AFL", "A", "APD", "AKAM", "ALK", "ALB", "ARE", "ALXN", "ALGN", "ALLE", "AGN", "ADS", "LNT",
	"ALL", "GOOGL", "GOOG", "MO", "AMZN", "AEE", "AAL", "AEP", "AXP", "AIG", "AMT", "AWK", "AMP",
	"ABC", "AME", "AMGN", "APH", "APC", "ADI", "ANDV", "ANSS", "ANTM", "AON", "APA", "AIV", "AAPL",
	"AMAT", "APTV", "ADM", "ARNC", "AJG", "AIZ", "T", "ADSK", "ADP", "AZO", "AVB", "AVY", "BHGE",
	"BLL", "BAC", "BAX", "BBT", "BDX", "BRK.B", "BBY", "BIIB", "BLK", "HRB", "BA", "BKNG", "BWA",
	"BXP", "BSX", "BHF", "BMY", "AVGO", "BF.B", "CHRW", "CA", "COG", "CDNS", "CPB", "COF", "CAH",
	"KMX", "CCL", "CAT", "CBOE", "CBRE", "CBS", "CELG", "CNC", "CNP", "CTL", "CERN", "CF", "SCHW",
	"CHTR", "CVX", "CMG", "CB", "CHD", "CI", "XEC", "CINF", "CTAS", "CSCO", "C", "CFG", "CTXS",
	"CME", "CMS", "KO", "CTSH", "CL", "CMCSA", "CMA", "CAG", "CXO", "COP", "ED", "STZ", "GLW",
	"COST", "COTY", "CCI", "CSX", "CMI", "CVS", "DHI", "DHR", "DRI", "DVA", "DE", "DAL",
	"XRAY", "DVN", "DLR", "DFS", "DISCA", "DISCK", "DISH", "DG", "DLTR", "D", "DOV", "DWDP", "DPS",
	"DTE", "DUK", "DRE", "DXC", "ETFC", "EMN", "ETN", "EBAY", "ECL", "EIX", "EW", "EA", "EMR",
	"ETR", "EVHC", "EOG", "EQT", "EFX", "EQIX", "EQR", "ESS", "EL", "RE", "ES", "EXC", "EXPE",
	"EXPD", "ESRX", "EXR", "XOM", "FFIV", "FB", "FAST", "FRT", "FDX", "FIS", "FITB", "FE", "FISV",
	"FLIR", "FLS", "FLR", "FMC", "FL", "F", "FTV", "FBHS", "BEN", "FCX", "GPS", "GRMN", "IT", "GD",
	"GE", "GGP", "GIS", "GM", "GPC", "GILD", "GPN", "GS", "GT", "GWW", "HAL", "HBI", "HOG", "HRS",
	"HIG", "HAS", "HCA", "HCP", "HP", "HSIC", "HES", "HPE", "HLT", "HOLX", "HD", "HON", "HRL",
	"HST", "HPQ", "HUM", "HBAN", "HII", "IDXX", "INFO", "ITW", "ILMN", "INCY", "IR", "INTC", "ICE",
	"IBM", "IP", "IPG", "IFF", "INTU", "ISRG", "IVZ", "IPGP", "IQV", "IRM", "JBHT", "JEC", "SJM",
	"JNJ", "JCI", "JPM", "JNPR", "KSU", "K", "KEY", "KMB", "KIM", "KMI", "KLAC", "KSS", "KHC",
	"KR", "LB", "LLL", "LH", "LRCX", "LEG", "LEN", "LUK", "LLY", "LNC", "LKQ", "LMT", "L", "LOW",
	"LYB", "MTB", "MAC", "M", "MRO", "MPC", "MAR", "MMC", "MLM", "MAS", "MA", "MAT", "MKC", "MCD",
	"MCK", "MDT", "MRK", "MET", "MTD", "MGM", "KORS", "MCHP", "MU", "MSFT", "MAA", "MHK", "TAP",
	"MDLZ", "MON", "MNST", "MCO", "MS", "MSI", "MYL", "NDAQ", "NOV", "NAVI", "NKTR", "NTAP",
	"NFLX", "NWL", "NFX", "NEM", "NWSA", "NWS", "NEE", "NLSN", "NKE", "NI", "NBL", "JWN", "NSC",
	"NTRS", "NOC", "NCLH", "NRG", "NUE", "NVDA", "ORLY", "OXY", "OMC", "OKE", "ORCL", "PCAR",
	"PKG", "PH", "PAYX", "PYPL", "PNR", "PBCT", "PEP", "PKI", "PRGO", "PFE", "PCG", "PM", "PSX",
	"PNW", "PXD", "PNC", "RL", "PPG", "PPL", "PX", "PFG", "PG", "PGR", "PLD", "PRU", "PEG", "PSA",
	"PHM", "PVH", "QRVO", "QCOM", "PWR", "DGX", "RRC", "RJF", "RTN", "O", "RHT", "REG", "REGN",
	"RF", "RSG", "RMD", "RHI", "ROK", "COL", "ROP", "ROST", "RCL", "SPGI", "CRM", "SBAC", "SCG",
	"SLB", "STX", "SEE", "SRE", "SHW", "SPG", "SWKS", "SLG", "SNA", "SO", "LUV", "SWK", "SBUX",
	"STT", "SRCL", "SYK", "STI", "SIVB", "SYMC", "SYF", "SNPS", "SYY", "TROW", "TTWO", "TPR",
	"TGT", "TEL", "FTI", "TXN", "TXT", "BK", "CLX", "COO", "HSY", "MOS", "TRV", "DIS", "TMO",
	"TIF", "TWX", "TJX", "TMK", "TSS", "TSCO", "TDG", "TRIP", "FOXA", "FOX", "TSN", "USB", "UDR",
	"ULTA", "UAA", "UA", "UNP", "UAL", "UNH", "UPS", "URI", "UTX", "UHS", "UNM", "VFC", "VLO",
	"VAR", "VTR", "VRSN", "VRSK", "VZ", "VRTX", "VIAB", "V", "VNO", "VMC", "WMT", "WBA", "WM",
	"WAT", "WEC", "WFC", "WELL", "WDC", "WU", "WRK", "WY", "WHR", "WMB", "WLTW", "WYN", "WYNN",
	"XEL", "XRX", "XLNX", "XL", "XYL", "YUM", "ZBH", "ZION", "ZTS", "SPY",
];

pub static STRATEGIES: [&str; 10] = [
	"hhv:<ticker>:<param>",
	"llv:<ticker>:<param>",
	"conupdays:<ticker>:<param>",
	"condowndays:<ticker>:<param>",
	"gapup:<ticker>:<param>",
	"gapdown:<ticker>:<param>",
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

pub static TARGET_TICKER: &str = "SPY";

// Mutate probably of dna applied after crossover
pub static MUTATE_PROB: f32 = 0.20;

// The number of fittest chromosomes to evolve
pub static FITTEST: usize = 500;

pub static POPULATION_SIZE: i32 = 10000;
