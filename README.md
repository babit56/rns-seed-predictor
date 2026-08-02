# Rabbit & Steel Seed Predictor

Have you ever wondered what items/shops/fights a seed has? Wonder no longer! Thanks to some solid reverse engineering by the [Rabbit & Steel Discord](https://discord.com/invite/mns), the main elements of the RNG used to generate a run can and has now been implemented in Rust. This has previously been done in GML, which this code is mostly based on.

## Usage

```sh
# Print out possible commandline arguments
rns-seed-predictor --help

# Generate and print out info for the seed 1585
rns-seed-predictor --seed 1585

# Generate all unique seeds for DLC, and search through the results
rns-seed-predictor --starting-area random-extra
cd seed_searcher/
python3 main.py ../unique_seeds.csv

# Generate unique seeds with only the Ruins and Darkbite sets unlocked and save the data in ruins-darkbite-seeds.csv
rns-seed-predictor --ruins --darkbite -o ruins-darkbite-seeds.csv

# Generate all possible combinations of unlocked sets where the ruins and darkbite sets are unlocked. Saves in full_gen/
rns-seed-predictor --full-generation --ruins --darkbite
```

See README in `seed_searcher/` folder for more on searching through the generated seeds

Generating a single csv takes ~17s on my machine (3.7GHz)

The `--full-generation` option uses multiple processes, as many as possible. At most one process is spawned per unlock combination. The above example runs in ~7 min on my machine (12 cores/threads @ 3.7GHz)


## Output

Though it's subject to change, at the time of writing the output of `rns-seed-predictor --seed 1585` is:
```
Using unlocks with following bitstring: 11111111111111111111
Seed: 1585
Players: 4
Difficulty: Hard/Lunar
Hallseeds: [2900418217, 2572066016, 97023707, 219281447, 1459899610, 657291952]
Starting area: RandomKingdom
Areas: [Kingdom Outskirts, Churchmouse Streets, Red Darkhouse, Scholar's Nest, Moonlit Prescipice]
Encounters: [enc_mouse_cadet2, enc_bird_sophomore2, enc_dragon_granite2, enc_mouse_archer1, enc_mouse_oakspear1, enc_mouse_rosemage0, enc_mouse_paladin0, enc_dragon_gold1, enc_dragon_emerald0, enc_dragon_ruby0, enc_dragon_mythril0, enc_bird_student1, enc_bird_whispering0, enc_bird_archon0, enc_bird_valedictorian0, enc_queens_spear0, enc_queens_staff0, enc_queens_axe0, enc_rabbit_queen0]

Shop 1:
  Gems: Garnet, Garnet, Opal, Opal
  Prices: 24g, 27g, 23g, 27g
  Potions: Regeneration Potion, Luck Potion, Essence of Steel
  Prices: 8g, 9g, 10g
Shop 2:
  Gems: Ruby, Sapphire, Sapphire, Sapphire
  Prices: 26g, 25g, 27g, 24g
  Potions: Winged Potion, Longarm Potion, Essence of Strength
  Prices: 10g, 7g, 9g
Shop 3:
  Gems: Garnet, Garnet, Emerald, Ruby
  Prices: 26g, 26g, 24g, 24g
  Potions: Essence of Strength, Essence of Steel, Swifthand Potion
  Prices: 8g, 9g, 9g
Shop 4:
  Gems: Emerald, Ruby, Opal, Ruby
  Prices: 27g, 25g, 27g, 23g
  Potions: Evasion Potion, Vitality Potion, Essence of Steel
  Prices: 7g, 10g, 7g

Chest 1 - Opal:
  Clockwork Tome, Moon Pendant, Sewing Sword, Sleeping Greatbow, Ghost Spear
Chest 2 - Ruby:
  Pocketwatch, Ruins Sword, Granite Greatsword, Robe of Light, Flame Bow
Chest 3 - Emerald:
  Aquamarine Bracelet, Raindrop Earrings, Venom Hood, Midsummer Dress, Seashell Shield
Chest 4 - White:
  Lonesome Pendant, Lancer Gauntlets, Bloodhound Greatsword, Butterfly Hairpin, Cursed Candlestaff
Chest 5 - Sapphire:
  Hidden Blade, Teacher Knife, Tactician Rod, Shockwave Tome, Windbite Dagger
Chest 6 - White:
  Emerald Chestplate, Glittering Trumpet, Bloody Bandage, Usagi Kamen, Snow Boots

1585,0,4,3,1,6,418,411,624,407,543,531,584,448,637,457,598,597,509,501,511,611,557,434,595,545,641,552,553,561,424,467,489,432,539,606,681,691,692,8,9,10,709,24,710,27,699,23,700,27,685,694,689,10,7,9,705,26,702,25,703,27,704,24,689,692,687,8,9,9,709,26,710,26,715,24,708,24,693,695,692,7,10,7,713,27,706,25,699,27,708,23
1585,hw_outskirts,hw_streets,hw_lighthouse,418,411,624,407,543,531,584,448,637,457,598,597,509,501,511,611,557,434,595,545,641,552,553,561,424,467,489,432,539,606,681,691,692,8,9,10,709,24,710,27,699,23,700,27,685,694,689,10,7,9,705,26,702,25,703,27,704,24,689,692,687,8,9,9,709,26,710,26,715,24,708,24,693,695,692,7,10,7,713,27,706,25,699,27,708,23
```

The 2nd to last line is meant for the csv, where all the numbers are various ID's. See constants at the top of `src/main.rs` for ID -> Item names and more.

## Building

Make sure you have installed cargo, for example via [rustup](https://rust-lang.org/tools/install/).
```
cargo build --release
cargo run --release
```
