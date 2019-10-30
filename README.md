# big-json-to-csv

## To Run

```bash
cargo run -- -i ./example_input_json.txt -o ./example_output_csv.csv
```

## Creating Mock Data

_Want to make 100K record file...or more?_

```
cargo play ./generate_big_input_json.rs -- -o ./blah.txt -r 100000
```

> Requires `cargo-play`
>
> Install it using the following:
>
> ```bash
> cargo install cargo-play --force
> ```
