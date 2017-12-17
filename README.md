# P.A.T. - Poloniex Analyzer Tool

This tool was written to further my learning of Rust.

I have tried my best to keep simple, concise code, while demonstrating several features of the language and the following goals:

1. Enumerations
2. Structures
3. Traits
4. Impls
5. Concurrency
6. Parallelization
7. Websockets
8. Graphics
9. Strong types
10. Borrow checker (memory management)
11. Create a lightweight Poloniex client.

Please feel free to fork this code and hack it to your hearts content.

Currently these are the features I track on Monero:

1. Last sell price
2. Current sell volume
3. Average time between sell trades

These metrics are useful to me because I can tell when a pump or dump may occur, or when a currency is seeing some activity.

The one thing this is missing is a simple plotting function.

This program uses circular queues to keep track of the last 10 things that have happened. That means the amount of memory this program uses is fixed.
