use backtester::core::broker::*;
use backtester::core::bt::*;
use backtester::dataframe::timeseries::*;
use backtester::strategy::sma_crossover::*;
use backtester::util::config::Config;

fn main() {
    let config = Config::new();
    println!("{:?}", config);

    let timeseries = TimeSeries::new(&config.root_directory);

    let strategy = SMACrossoverStrategy::new(10);
    let broker = Broker::new("Test", 10000.0, 0.02, 0.2, false, false);
    let backtest = Backtest::new(timeseries, broker, Box::new(strategy));
    let results = backtest.run();

    println!("{:?}", results);
}
