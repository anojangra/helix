use uuid::Uuid;
use chromosome::Chromosome;
use std::collections::BTreeMap;
use trade_signal;
use trade_signal::TradeSignal;
use schemas::Quote;



pub mod highest_high_value;
pub mod lowest_low_value;

#[derive(Debug)]
#[derive(Clone)] 
pub struct Strategy {
    pub chromosome_id: Uuid,
    pub strategy: String,
    pub code: String,
    pub ticker: String,
    pub param: i64,
    pub target_ticker: String,
    pub generation: i64,
}

/// Expands chromosome of strategies to a list of strategies
///
/// "llv:AAPL:2::gapupday:GOOG:10"
///
/// Returns
/// [
///     Strategy {
///         name: "llv",
///         ticker: "AAPL",
///         param: 2
///     },
///     Strategy {
///         name: "gapupday",
///         ticker: "GOOG",
///         param: 10
///     }
/// ]
///
pub fn expand_strategies(chromosome: &Chromosome) -> Vec<Strategy> {
    let strategies: Vec<&str> = chromosome.chromosome.split("::").collect();
    let expanded_strategies = strategies
        .into_iter()
        .map(|s| expand_strategy(chromosome, s.to_string()))
        .collect();
    expanded_strategies
}

/// Expands chrosomes to Strategy
pub fn expand_strategy(chromosome: &Chromosome, strategy: String) -> Strategy {
    let v: Vec<&str> = strategy.split(":").collect();
    let strategy_name = strategy.clone();
    Strategy {
        chromosome_id: chromosome.id,
        strategy: strategy_name,
        code: v[0].to_string(),
        ticker: v[1].to_string(),
        param: v[2].parse::<i64>().unwrap(),
        target_ticker: chromosome.target_ticker.clone(),
        generation: chromosome.generation,
    }
}

fn insert_signal(
    trade_signals: BTreeMap<String, TradeSignal>,
    quote: &Quote,
    strategy: Strategy,
    signal: i64
) -> BTreeMap<String, TradeSignal> {
    let mut signals = trade_signals;
    let trade_signal = match signals.get(&quote.ts.to_string()) {
            Some(s) => update_signal(s, strategy, signal),
            None => trade_signal::init_trade_signal(strategy, &quote, signal),
        };
    signals.insert(quote.ts.to_string(), trade_signal);
    signals
}


fn update_signal(trade_signal: &TradeSignal, strategy: Strategy, signal: i64) -> TradeSignal {
    let mut strategies = trade_signal.strategies.clone();
    strategies.push(strategy.strategy);
    let mut signals = trade_signal.signals.clone();
    signals.push(signal);
    TradeSignal {
        chromosome_id: trade_signal.chromosome_id,
        ts: trade_signal.ts,
        strategies: strategies,
        signals: signals,
        target_ticker: trade_signal.target_ticker.clone(),
        hard_signal: trade_signal.hard_signal,
        generation: trade_signal.generation,
    }
}

#[cfg(test)]
#[test]
fn test_expand_strategy() {
    let expected = Strategy {
        name: String::from("llv"),
        ticker: String::from("krakenUSD"),
        param: 2,
    };

    let actual = expand_strategy("llv:krakenUSD:2".to_string());

    assert_eq!(expected.name, actual.name);
    assert_eq!(expected.ticker, actual.ticker);
    assert_eq!(expected.param, actual.param);
}

#[test]
fn test_expand_strategies() {
    let actual = expand_strategies("llv:krakenUSD:2::hhv:krakenUSD:5".to_string());

    assert_eq!("llv".to_string(), actual[0].name);
    assert_eq!("krakenUSD".to_string(), actual[0].ticker);
    assert_eq!(2, actual[0].param);
}
