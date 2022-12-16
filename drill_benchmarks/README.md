## Benchmarkting http services for kafka ingestion

download the drill tool here: 

`https://github.com/fcsonline/drill`

and build it from source with 

`cargo build --release`

Run the drill tool to benchmark the REST API: 

`./drill  --benchmark benchmark_node.yml -s`
