<h1 align="center">Shape Seed Collector</h1>

## What is This?
This is a Rust program that collects the different hashes of shape by pinging an endpoint
The example here is target

## Getting Started
This should be *plugNplay* for target but of course you can modify the script endpoint and it should play nice
I have yet to test other sites at the moment and pull requests are very much open
**1. Clone the repo** `https://github.com/raakka/seed-collector && cd seed-collector`
**2. Run** `cargo run --release`

## Output

### Files
When you run the program it should make a JSON file `json-seed/greenhouse.json`\
It should also save each script in a vanilla format in a folder called `/seed-output`

### Format
Each seed should have a JSON obj like this
```json
{
  "seed": String
  "v": String
  "i": String
}
```
