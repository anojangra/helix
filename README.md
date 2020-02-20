# Rusty Relix

## January 2020

Coming back to this code after two years. Trying to figure out how to split the workload up across mutilple servers. 

Seems like the heavy lifting occurs in the `vger` module. Everything else scaffolds and manages the concurrency. 

> Can we just move the concurrency and data management to an elixir application? 

Architecturally, we need to figure out how to move the data. Some things to consider:

* Should we use ETS at each node? 
* What should get passed to each node? 
* Start with getting the data from postgresql.

## Things to abstract

1. `forge` - ticker pool 
2. 