# Rabbit & Steel Seed Predictor
Predict items, shops, fights, etc. from a seed in hit game Rabbit & Steel.

Missing docs, tests, error checking. Probably won't be added unless it's wanted or I'm bored

## Usage
```rust
let run = rnssp::Run::new(
  1585, // Seed
  4, // Players
  true, // is Hard/Lunar
  rnssp::Unlocks::with_all(), // What sets are unlocked
);

run.predict_seed();

// Output run data
println!(run.get_csv_line());
let first_area = rnssp::names::get_area_name(run.area_list[0]);
println!("First area: {}", first_area);
```

You can also use other unlocks, e.g. `Unlocks::with_none()` or `Unlock::from_bitstring()`. Bitflags correspond to the the order that they appear on [the wiki](https://rns.miraheze.org/wiki/Loot), i.e. the least significant bit is darkbite etc.
