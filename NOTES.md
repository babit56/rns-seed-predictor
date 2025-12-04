# Notes on RNG
See (point to gm rng header) for notes on the engine RNG. In short, there's initially 2^16 unique seeds, given that you only use one seed. RnS however uses *two* seeds: the main seed and seed+5. There's two possible outcomes: either these give the same PRNG state, or they give different states. This results in there being 2*2^16 unique seeds/runs for RnS. Notably, items only use seed+5 for generation, so there's only 2^16 unique combinations of items.
TODO: Loot generation affects shop generation and other objects. Cute/normal gives different first shop. Fewer players gives less items (notably, not different items).

Need more info: seeds for each fight, i.e. are crits and other mid-fight luck, based on mapseed? (I think it isn't, see some calls to randomise())

## Gamemaker RNG
TL;DR: They use WELL512a as the PRNG, and a TLCG for generating the inital state of WELL512a. Both implementations are taken from Lomont 2008 (insert source). The TLCG is implemented wrong (see source code here for the current and a "correct" impl). What it should be doing is using the seed as the inital state, iterate the state with an LCG, then it should output a truncation of its state. What the (legacy and HTML) GML impl does instead is truncating when iterating the state, which means that the state will only have 15 bits of information, giving at most 2^15 unique seeds. In current Windows and Linux GML, they have also removed some of the truncation, so the state and outputs have 16 bits of information. I have no source for this, but I'm guessing they noticed the problem and put on a bandaid by removing some truncation. Lol. 
With 16 bits of information stored for the TLCG, there's in total 2^16 possible inital states of WELL512a which gives 2^16 unique seeds.
The small output space of the TLCG luckily does not make the WELL512a output significantly worse (given enough runtime). As far as I understand, having lots of zeros in state is called "zeroland", and WELL512a is very good at escaping this quickly.

## Algorithm
Includes what I know about so far. To be included later: choosing patterns for areas, difference in demo (maybe), info on seeds for each fight being included

Start with a seed (named mapseed), generate 6 hallseeds, used for future generation of each area. Areas are shuffled between the 1st and 2nd hallseed are gotten.
Items:
  Start RNG with mapseed+5
  Shuffle all item lists
  Add Opal/Sapphire etc sets to their respective colored chests
  Shuffle all item lists
  Move some items from colored chests to white chest
  Shuffle all item lists
  Shuffle chest list twice
  Get 6 chests with items
Outskirts:
  Start RNG with the 1st hallseed
  Pick between two patterns for all 5 opponents
  Shuffle list of opponents
Areas:
  Start RNG with 2nd, 3rd etc hallseed
  If pale keep, shuffle list of potential opponents
  Start RNG with the output of random(2^31-1) (No idea why)
  Shuffle potions
  Change first potion to regen when hard/lunar in area 1
  Pick price of 3 potions, or 2 if regen potion was used
  For all 4 abilities:
    Shuffle gem upgrade list
    Pick price

## Items
What (I think) RnS calls "items" are various objects a player can obtain. This includes loot, potions, specific ability gems, generic ability upgrade gems, heal, lvl up, and some placeholders. Presumably modded loot/potions/gems and future dlc loot/potions will also be in this category. Notably, areas, fights and chests are NOT in this category.
All the items are kept in a big list (`OBJECT_NAMES` in this repository), and the ID of an item is the index in this list.
All these ID's are also kept in a big list of lists

### The list of all list
The ID for the objects mentioned above are all kept in a list of lists, roughly sorted by "type". All these lists are shuffled a couple times during item generation, which also affects shop generation (since gems and potions are kept here)
This list contains the following

1. Leftovers - big list that contains everything not used in other lists. Includes various placeholders, lvl up, heal, all gem abilities, etc
2. Empty - No idea why it's here, could be removed since it doesn't affect RNG
3. White chest - Used to store items in white chests. Starts empty
4. Opal chest - Used for opal chest. Starts with the two item sets that are only in opal chests
5. Sapphire chest - See 4.
6. Ruby chest - See 4.
7. Garnet chest - See 4.
8. Emerald chest - See 4.
9. Opal/Sapphire set - Contains the set that will go in opal or sapphire chests
10. Opal/Ruby set - See 9.
11. Opal/Garnet set - See 9.
12. Opal/Emerald set - See 9.
13. Sapphire/Ruby set - See 9.
14. Sapphire/Garnet set - See 9.
15. Sapphire/Emerald set - See 9.
16. Ruby/Garnet set - See 9.
17. Ruby/Emerald set - See 9.
18. Garnet/Emerald set - See 9.
19. Regenation potion - Only keeps regen potion, presumably because it should not be mixed with other potions
20. Other potions - Has all the potions other than regen
21. Primary gem upgrades - Has e.g. Opal Primary Upgrade, used in shop generation
22. Secondary gem upgrades - See 21.
22. Special gem upgrades - See 21.
22. Defensives gem upgrades - See 21.
