# Rabbit & Steel Seed Predictor

Have you ever wondered what items/shops/fights a seed has? Wonder no longer! Thanks to some solid reverse engineering by the [RnS discord](https://discord.com/invite/mns), the main elements of the RNG used to generate a run can and has now been implemented in Rust. This has previously been done in GML, which this code is mostly based on.

## Usage

The executable accepts up to 3 parameters: `seed`, `players` and `high_difficulty`. The last parameter distinguishes between Cute/Normal and Hard/Lunar. 
Default values are:

```sh
rns-seed-predictor 1585 4 true
```

Though it's subject to change, at the time of writing the output is:
```
Seed: 1585
Players: 4
Difficulty: Hard/Lunar
Hallseeds: [1521834084,265854828,29165545,1800124766,1137214511,3340410779]
Areas: [hw_nest,hw_lighthouse,hw_lakeside,hw_arsenal,hw_streets]
Outskirts: [enc_bird_sophomore1,enc_frog_tinkerer2,enc_wolf_blackear2,enc_mouse_cadet1,enc_dragon_granite1]
Pale Keep: [enc_queens_knife0,enc_queens_staff0,enc_queens_axe0,enc_queens_harp0,enc_queens_spear0]

Shop 0:
  Gems: Sapphire Primary Upgrade, Garnet Secondary Upgrade, Emerald Special Upgrade, Garnet Defensive Upgrade
  Prices: 23, 24, 23, 27
  Potions: Regeneration Potion, Essence of Spell, Fire Potion
  Prices: 8, 9, 10
Shop 1:
  Gems: Ruby Primary Upgrade, Ruby Secondary Upgrade, Ruby Special Upgrade, Opal Defensive Upgrade
  Prices: 27, 24, 26, 26
  Potions: Essence of Spell, Fire Potion, Darkness Potion
  Prices: 7, 8, 10
Shop 2:
  Gems: Ruby Primary Upgrade, Sapphire Secondary Upgrade, Ruby Special Upgrade, Emerald Defensive Upgrade
  Prices: 25, 24, 26, 23
  Potions: Darkness Potion, Essence of Strength, Quickening Potion
  Prices: 8, 9, 10
Shop 3:
  Gems: Ruby Primary Upgrade, Emerald Secondary Upgrade, Emerald Special Upgrade, Garnet Defensive Upgrade
  Prices: 25, 25, 25, 25
  Potions: Vitality Potion, Longarm Potion, Essence of Wit
  Prices: 8, 10, 9

Chest 0 - White:
  Items: Blacksteel Buckler, Thunderclap Gloves, Bloodflower Brooch, Mountain Staff, Battlemaiden Armor
Chest 1 - Opal:
  Items: Firststrike Bracelet, Phantom Dagger, Darkglass Spear, Gemini Necklace, Curse Talon
Chest 2 - Ruby:
  Items: Granite Greatsword, Ruby Circlet, Desert Earrings, Tough Gauntlet, Sapphire Violin
Chest 3 - White:
  Items: Giant Stone Club, Garnet Staff, Shinobi Tabi, Obsidian Hairpin, Reflection Shield
Chest 4 - Sapphire:
  Items: Shockwave Tome, Talon Charm, Vampiric Dagger, Hawkfeather Fan, Sniper's Eyeglasses
Chest 5 - White:
  Items: Leech Staff, Tiny Hourglass, Ruins Sword, Ivy Staff, Ornamental Bell

1585,hw_nest,hw_lighthouse,hw_lakeside,hw_arsenal,hw_streets,enc_bird_sophomore1,enc_frog_tinkerer2,enc_wolf_blackear2,enc_queens_knife0,enc_queens_staff0,enc_queens_axe0,410,365,324,473,443,414,432,416,310,289,336,358,470,339,354,471,353,334,341,482,449,460,319,311,412,321,469,472,392,370,489,490,496,8,9,10,508,23,517,24,522,23,519,27,490,496,491,7,8,10,512,27,513,24,514,26,507,26,491,497,492,8,9,10,512,25,509,24,514,26,523,23,503,502,494,8,10,9,512,25,521,25,522,25,519,25,
```

The last line is meant for a csv, where all the numbers are various ID's. See constants at the top of `src/main.rs` for ID -> Item names and more.

## Building

Make sure you have installed cargo, for example via rustup.
```
cargo build --release
cargo run --release
```
Currently the code does not run in debug mode since Rust panics on integer overflows (which are used frequently in the RNG code). I might fix this later, we'll see
