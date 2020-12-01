# advent of code 2020

## How to run?

1. install [cargo-aoc](https://github.com/gobanos/cargo-aoc)
2. provide cargo-aoc with credentials 
   ```
   cargo aoc credentials -s <your advent of code session cookie>
   ```
3. download your input
   ```
   cargo aoc input
   ```
4. run
   ```
   # run a specific day/part
   cargo aoc -d <day> -p <part>
   
   # run everything
   cargo aoc
   
   # run benchmarks
   cargo aoc bench -d <day> -p <part> 
   ```
