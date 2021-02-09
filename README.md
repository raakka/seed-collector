<h1 align="center">Shape Seed Collector</h1>

## What is This?
This is a Rust program that collects the different variants of shape by pinging an endpoint
The default for this repo is target but can be easily changed

## Getting Started
This should be plug and play for target but of course you can modify the script endpoint and it should play nice
I have yet to test other sites at the moment and pull requests are open

**1. Clone the repo** `https://github.com/raakka/seed-collector && cd seed-collector`\
**2. Run** `cargo run --release`

## Output

### Files
When you run the program it should make a JSON file `json-seed/greenhouse.json`

### Format
Each seed should have a JSON obj like this
```javascript
{
  "seed": String
  "v": String
  "i": String
}
```
