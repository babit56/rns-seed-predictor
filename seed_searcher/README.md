# Seed searcher
See yaml for config
Full credits to dotdot for the original implementation
Uses as many processes (cores/threads) as possible, with at most 1 process per csv

## Usage
Install [yaml](https://pyyaml.org/wiki/PyYAMLDocumentation), via e.g. `pip install pyyaml`.
Edit `config.yaml` to desired search parameters. As an example it currently searches for Blackstrike/Mountain staff in the first two chests (so you get both) and g2/e3 at affordable prices in the first shop

```sh
python3 main.py
python3 main.py path/to/seeds.csv
```

Reads csv files from `full_gen/` if no args are given, otherwise reads from the first arg. Matching seeds are output in a readable format in `matching_runs_readable.txt` and in csv format in either `full_search_results/<bitstring>.csv` or `matching_runs.csv` depending on how the program was run

Searching through 256 csv's (all darkbite/ruins combinations) takes ~16s on my machine (12 cores/threads @ 3.7GHz)
