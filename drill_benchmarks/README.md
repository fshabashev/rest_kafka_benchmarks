## Benchmarkting http services for kafka ingestion

download the drill tool here: 

`https://github.com/fcsonline/drill`

and build it from source with 

`cargo build --release`

Run the drill tool to benchmark the REST API: 

`./drill  --benchmark benchmark_node.yml -s`


Results of the benchmarks: 

For Rust: 
`
Time taken for tests      1.1 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       9072.92 [#/sec]
Median time per request   99ms
Average time per request  102ms
Sample standard deviation 57ms
99.0'th percentile        139ms
99.5'th percentile        205ms
99.9'th percentile        1073ms
`

For node.js:

`
Time taken for tests      1.7 seconds
Total requests            10000
Successful requests       9666
Failed requests           334
Requests per second       5863.25 [#/sec]
Median time per request   161ms
Average time per request  159ms
Sample standard deviation 40ms
99.0'th percentile        232ms
99.5'th percentile        233ms
99.9'th percentile        234ms
`
