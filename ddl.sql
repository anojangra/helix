DROP TABLE IF EXISTS trade_signals;
CREATE TABLE trade_signals (
    chromosome_id uuid
,   ts integer not null
,   stratgies text array
,   signals integer array
,   target_ticker text
,   hard_signal integer
,   generation integer
,   ret numeric
,   pnl numeric
);
SELECT create_hypertable('trade_signals', 'ts', 'chromosome_id');
CREATE INDEX ON trade_signals (chromosome_id, ts);

-- pub struct TradeSignal {
--     pub chromosome_id: Uuid,
--     pub ts: f64,
--     pub strategies: Vec<String>,
--     pub signals: Vec<i32>,
--     pub target_ticker: String,
--     pub hard_signal: i32,
--     pub generation: i32,
--     pub ret: f32,
--     pub pnl: f32,
-- }

DROP TABLE IF EXISTS trade_chromosomes;
CREATE TABLE trade_chromosomes (
  id uuid,
  target_ticker text, 
  chromosome text,
  dna integer array,
  generation int,
  chromosome_length int,
  kelly numeric,
  cum_pnl numeric,
  variance numeric,
  mean_return numeric,
  w_kelly numeric,
  num_of_trades integer,
  rank integer
);

-- pub struct Chromosome {
--     pub id: Uuid,
--     pub target_ticker: String,
--     pub chromosome: String,
--     pub dna: Vec<i32>,
--     pub generation: i32,
--     pub chromosome_length: i32,
--     pub kelly: f32,
--     pub cum_pnl: f32,
--     pub variance: f32,
--     pub mean_return: f32,
--     pub w_kelly: f32,
--     pub num_of_trades: i32,
--     pub rank: i32,
-- }