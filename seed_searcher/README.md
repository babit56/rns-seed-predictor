# Seed searcher
See yaml for config
Full credits to dotdot for the original implementation

## Usage
Edit `config.yaml` to desired search parameters. As an example it currently searches for Blackstrike/Mountain staff in the first two chests (so you get both) and g2/e3 at affordable prices in the first shop

To do the search, run `python3 main.py`. Reads csv files from `full_gen/` at the moment. Matching seeds are output in a readable format in `matching_runs.txt`.

A ruins/darkbite search takes ~100s on my machine
