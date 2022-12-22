## Benchmarkting http services for kafka ingestion

download the drill tool here: 

`https://github.com/fcsonline/drill`

and build it from source with 

`cargo build --release`

Run the drill tool to benchmark the REST API: 

`./drill  --benchmark benchmark_node.yml -s`


Results of the benchmarks: 

For Rust: 
```
Time taken for tests      1.0 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       10405.54 [#/sec]
Median time per request   8ms
Average time per request  9ms
Sample standard deviation 3ms
99.0'th percentile        19ms
99.5'th percentile        22ms
99.9'th percentile        27ms
```
---

For node.js:

```

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
```

---

results from go
```
Time taken for tests      0.2 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       43890.45 [#/sec]
Median time per request   1ms
Average time per request  1ms
Sample standard deviation 1ms
99.0'th percentile        3ms
99.5'th percentile        6ms
99.9'th percentile        9ms

```

---

results from FastAPI:


```
uvicorn server:app --port 5006
```

```
Time taken for tests      0.5 seconds
Total requests            1000
Successful requests       1000
Failed requests           0
Requests per second       2017.09 [#/sec]
Median time per request   41ms
Average time per request  47ms
Sample standard deviation 18ms
99.0'th percentile        104ms
99.5'th percentile        119ms
99.9'th percentile        125ms

```

---
results from Flask (via Waitress):
```
Time taken for tests      11.0 seconds
Total requests            300
Successful requests       300
Failed requests           0
Requests per second       27.21 [#/sec]
Median time per request   733ms
Average time per request  713ms
Sample standard deviation 116ms
99.0'th percentile        930ms
99.5'th percentile        946ms
99.9'th percentile        950ms
```
---
results for FastAPI (via guniorn):
command for reference:
```
gunicorn server:app --workers 8  --worker-class uvicorn.workers.UvicornWorker --bind 0.0.0.0:5002
```

```
Time taken for tests      0.8 seconds
Total requests            10000
Successful requests       10000
Failed requests           0
Requests per second       12991.51 [#/sec]
Median time per request   4ms
Average time per request  6ms
Sample standard deviation 9ms
99.0'th percentile        37ms
99.5'th percentile        64ms
99.9'th percentile        125ms
```
