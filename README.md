# Helix

An implementation of a genetic algortihm that uses grammatical evolution to find trading signals

## Build

```
cargo build --release
sudo mv target/release/helix /usr/local/bin
RUST_LOG=info ./helix
```

## Usage

```
RUST_LOG=debug ./helix --pool_description btc-exchanges --repo_pathname /home/choiway/data-repo/btc_prices_hourly/ -r coinbaseUSD_returns.csv --target_ticker coinbaseUSD --threads 12
```


```
USAGE:
    helix --pool_description <DESCRIPTION> --repo_pathname <PATH> -r <FILENAME> --target_ticker <TARGET_TICKER> --threads <THREADS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --pool_description <DESCRIPTION>    Description of the pool of securities (i.e. SP500, btc-exchanges)
    -p, --repo_pathname <PATH>              Path to work directory. Should have a *data* directory as a sub directory
    -r <FILENAME>                           Filename of the target returns to predict. Should be located in the repo
    -s, --target_ticker <TARGET_TICKER>     The ticker of the security you are trying to predict (i.e. SPY, AAPL,
                                            coinbaseUSD)
    -t, --threads <THREADS>                 Sets the number of threads to use
```

### Data Repo

The data repo should be structured as follows:

```
+-- data-repo - contains script to gather data
    SPY_returns.txt (-r)
    +-- data - Helix expects this directory to exist in the repo pathname
    +-- backtests - results are saved to /tmp but can be manually moved here
```

#### SPY_returns.txt

Data in this file should have the following shape: 

```
epoch, return
```

The first column represents epoch time and the second column represents the return to predict. This is usually the next period return.  

#### data

All historical price data used in the anlaysis should be stored here. Helix will read the entire directory and parse the ticker symbol from the file name so files should be names accordingly. For example the file for `AAPL` should have a `AAPL.csv` filename. 

Data in the file should follow the `OHLCV` format without headers. The first column should be the ticker symbol and the second column should be epoch time. 

```
AAPL,953510400,123.5,126.25,122.37,123.0,1825800
AAPL,953596800,122.56,136.75,121.62,134.94,4681500
AAPL,953683200,132.78,144.38,131.56,144.19,5071400
AAPL,953769600,142.0,150.38,140.0,141.31,5022900
AAPL,953856000,142.44,143.94,135.5,138.69,3990300
AAPL,954115200,137.63,144.75,136.88,139.56,2492700
AAPL,954201600,137.25,142.0,137.13,139.13,1812200
AAPL,954288000,139.38,139.44,133.83,135.94,2141400
AAPL,954374400,133.56,137.69,125.44,125.75,3700000
AAPL,954460800,127.44,137.25,126.0,135.81,3612800
```

## Results

### Trade Pnl of trading strategies

Each trading strategy is saved in a separate file with each row representing a TradePnl with the backtest id added at the beginning of the row.

```
pub struct TradePnl {
    pub chromosome_id: uuid::Uuid,
    pub ts: chrono::DateTime<Utc>,
    pub strategies: Vec<String>,
    pub ticker: String,
    pub signal: i32,
    pub pnl: f32,
    pub cum_pnl: f32,
    pub generation: i32,
}
```

Head of sample output:

```
qqq::QQQ-1584936501.899630481s	fffea29d-a468-4129-9cb5-2ba958c8682b	1000080000	{gapdown:SIRI:195,condowndays:ADI:255}	{0,0}	QQQ	0	1	-0.08504481	0
qqq::QQQ-1584936501.899630481s	fffea29d-a468-4129-9cb5-2ba958c8682b	1000684800	{gapdown:SIRI:195,condowndays:ADI:255}	{0,0}	QQQ	0	1	-0.022435756	0
qqq::QQQ-1584936501.899630481s	fffea29d-a468-4129-9cb5-2ba958c8682b	1000771200	{gapdown:SIRI:195,condowndays:ADI:255}	{0,0}	QQQ	0	1	-0.017376458	0
qqq::QQQ-1584936501.899630481s	fffea29d-a468-4129-9cb5-2ba958c8682b	1000857600	{gapdown:SIRI:195,condowndays:ADI:255}	{0,0}	QQQ	0	1	-0.033367552	0
qqq::QQQ-1584936501.899630481s	fffea29d-a468-4129-9cb5-2ba958c8682b	1000944000	{gapdown:SIRI:195,condowndays:ADI:255}	{0,0}	QQQ	0	1	-0.026924014	0
```

### Generation (Gene Pool)

Each generation is saved in a separate file with each row representing a Chromosome with a the backtest id added at the beginning of the row.

```
pub struct Chromosome {
  pub id: Uuid,
  pub target_ticker: String,
  pub chromosome: String,
  pub dna: Vec<i32>,
  pub generation: i32,
  pub chromosome_length: i32,
  pub kelly: f32,
  pub cum_pnl: f32,
  pub variance: f32,
  pub mean_return: f32,
  pub w_kelly: f32,
  pub num_of_trades: i32,
  pub winning_trades: i32,
  pub losing_trades: i32,
  pub percentage_winners: f32,
  pub rank: i32,
}
```

Head of sample output:

```
qqq::QQQ-1584936501.899630481s	dc9691ad-67c9-405e-94b1-7e0bf52f85a3	QQQ	stdeva:CHKP:79	{228,196,233,79,82,210,107,166,229,61,123,67}	3	1	0.4727643	0.023151325	0.00034730582	0.0001641938	0.013458463	141	85	55	0.6028369	0
qqq::QQQ-1584936501.899630481s	0eeb9d7c-8537-42fb-94b2-620d0c1bb571	QQQ	hhv:VRSN:43	{116,240,202,43,195,91,18,129,199,177,75,108}	3	1	1.5220568	0.0491798	0.00011027785	0.00016784915	0.08938919	293	157	135	0.53583616	0
qqq::QQQ-1584936501.899630481s	103a962c-42b3-4d97-ba9c-24cc7d232cb4	QQQ	stdeva:ASML:25	{138,36,118,25,3,195,14,233,64,167,32,220}	3	1	-5.671136	-0.20119064	0.00020991865	-0.0011904772	-0.19141641	169	93	75	0.5502958	0
qqq::QQQ-1584936501.899630481s	1b779947-db04-4238-bd7d-56d0fb71957a	QQQ	stdeva:AMGN:30	{102,206,220,30,203,122,36,164,133,170,123,152}	3	1	-8.153626	-0.4086216	0.00027535894	-0.0022451738	-0.29667333	182	92	90	0.50549453	0
qqq::QQQ-1584936501.899630481s	0221aa5e-e445-4085-8ca9-1a59e726ef9d	QQQ	stdevb:CTAS:146	{212,137,31,146,152,1,30,80,61,155,43,87}	3	1	2.5284703	0.33157846	0.00024836737	0.00062798953	0.27323624	528	296	231	0.56060606	0
```