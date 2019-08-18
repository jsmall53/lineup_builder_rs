# LineupBuilder

LineupBuilder is a tool for constructing optimized DFS lineups. Currently only supports
[Draft Kings](https://www.draftkings.com), but the plan is to support all major providers.

The optimization engine is located in the [builder](https://github.com/jsmall53/lineup_builder_rs/tree/master/builder) subcrate. There are currently two optimizer backends available. One is a custom knapsack algorithm implementation that I would not recommend using. The second, and recommended optimizer, is a wrapper around the [coinor-cbc](https://github.com/coin-or/Cbc) tool using the [lp-modeler](https://github.com/jcavat/rust-lp-modeler) crate.

The main crate is an unfinished console application.
