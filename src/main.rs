use backtester::core::bt;
use backtester::dataframe::timeseriesbatch::TimeSeriesBatch;
use backtester::util::config::Config;

fn main() {
    let config = Config::new();
    println!("{:?}", config);

    // let batch = TimeSeriesBatch::load_from_folder(&config.root_directory);
    // for stock in batch.data {
    // bt::run(stock, SimpleSMA::new(), 10000.0, 0.02);
    // }

    // for mut stock in batch.data {
    //     for ticker in stock {
    //         println!("{:?}", ticker);
    //     }
    // }
    //     println!("{:?}", stock.buffer.next());
    //     println!("{:?}", stock.buffer.next());
    //     println!("{:?}", stock.buffer.next());
    //     println!("{:?}", stock.buffer.next());
    // for line in stock.buffer {
    //     println!("{:?}", line);
    // }
    // }
    // let results = bt::run_batch(&batch);
    // println!("{:?}", batch);
}
