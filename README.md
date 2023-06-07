# High Performance Backtester

## Features
- Single Strategy 
- Multi-Strategy, Parallelized
- Hyperparameter Tuner 
- Simple Graphing
- Logging Actions
- Simulated Exchange
- Comparing Results Against Other Investments


## Formatting
Please ensure that CSV files fed to the `TimeSeries::from_path` constructor contain the columns
- `open`
- `high`
- `low`
- `close`
- `volume`
- `datetime`
Otherwise, deserializing the CSV file will fail.