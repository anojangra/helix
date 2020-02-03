# Rusty Relix

## January 2020

Coming back to this code after two years. Trying to figure out how to split the workload up across mutilple servers. 

Seems like the heavy lifting occurs in the `vger` module. Everything else scaffolds and manages the concurrency. 

> Can we just move the concurrency and data management to an elixir application? 

Architecturally, we need to figure out how to move the data. Some things to consider:

* Should we use ETS at each node? 
* What should get passed to each node? 
* Start with getting the data from postgresql.

## Set Up

To run helix you need to pass the following paramaters:

* path to the data repo
* backtest id
* the path to the a csv file of the target's returns
* the path to the tickers that make up the "gene" pool

### Data Repo

The data repo should be structured as follows

/ data-repo - contains script to gather data
    returns.txt
    tickers.txt 
    / data - a repository for quotes.csv for each ticker (MMM should have a MMM.csv quotes file)
    / backtests - a repo where all results will be moved to from /tmp.

#### returns.txt

Data in this file should take the follow shape: 

```
epoch, return
```

The first column represents epoch time and the second column represents the next day's return for the target ticker. 
