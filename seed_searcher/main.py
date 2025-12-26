from dataclasses import dataclass
from typing import Generator
import yaml
import re
import itertools
import os
import sys

@dataclass
class Chest:
    contents: set[str]

@dataclass
class Upgrade:
    gem: str
    cost: int

@dataclass
class Potion:
    name: str
    cost: int

@dataclass
class Shop:
    upgrades: tuple[Upgrade, ...]
    potions: tuple[Potion, ...]

@dataclass
class Run:
    seed: int
    shops: tuple[Shop, ...]
    chests: tuple[Chest, ...]
    areas: tuple[str, ...]
    
    def __str__(self) -> str:
        lines = [f"Seed {self.seed}", ""]
        
        # Print chests
        for i, chest in enumerate(self.chests, 1):
            lines.append(f"CHEST {i}")
            for item in sorted(chest.contents):
                lines.append(item)
            lines.append("")
        
        # Print shops
        for i, shop in enumerate(self.shops, 1):
            lines.append(f"SHOP {i}")
            for potion in shop.potions:
                lines.append(f"{potion.name} {potion.cost}g")
            for upgrade in shop.upgrades:
                lines.append(f"{upgrade.gem} {upgrade.cost}g")
            lines.append("")
        
        return "\n".join(lines)

# Normal ID's
id_to_name: dict[int, str] = {}
name_to_id: dict[str, int] = {}

with open("ids.txt", 'r') as f:
    for line in f:
        match = re.match(r'\[(\d+)\s+"(.+)"\]', line)
        if match:
            id = int(match.group(1))
            name = match.group(2)
            id_to_name[id] = name
            name_to_id[name] = id

# Homemade ID's
id_area_list = [
    (0, "hw_nest"),
    (1, "hw_arsenal"),
    (2, "hw_lighthouse"),
    (3, "hw_streets"),
    (4, "hw_lakeside"),
]
id_to_area: dict[int, str] = {}
area_to_id: dict[str, int] = {}
id_outskirt_list = [
    (11, "enc_bird_sophomore1"),
    (12, "enc_bird_sophomore2"),
    (21, "enc_frog_tinkerer1"),
    (22, "enc_frog_tinkerer2"),
    (31, "enc_dragon_granite1"),
    (32, "enc_dragon_granite2"),
    (41, "enc_wolf_blackear1"),
    (42, "enc_wolf_blackear2"),
    (51, "enc_mouse_cadet1"),
    (52, "enc_mouse_cadet2"),
]
id_to_outskirt: dict[int, str] = {}
outskirt_to_id: dict[str, int] = {}

for (id, name) in id_area_list:
    id_to_area[id] = name
    area_to_id[name] = id

for (id, name) in id_outskirt_list:
    id_to_outskirt[id] = name
    outskirt_to_id[name] = id

def parse_line(line: str) -> Run:
    line_data = line.strip().split(',')
    seed = line_data[0]
    areas = tuple(line_data[1:4])
    # outskirts here
    chests = tuple(
        Chest(
            contents=set(
                id_to_name[int(item_id)]
                for item_id in chest_data
            )
        ) for chest_data in itertools.batched(line_data[7:37], 5)
    )
    shops = tuple(
        Shop(
            upgrades=tuple(
                Upgrade(
                    gem=id_to_name[int(upgrade_data[0])],
                    cost=int(upgrade_data[1])
                ) for upgrade_data in itertools.batched(shop_data[6:], 2)
            ),
            potions=tuple(
                Potion(
                    name=id_to_name[int(name)],
                    cost=int(cost)
                ) for name, cost in zip(shop_data[:3], shop_data[3:6])
            )
        ) for shop_data in itertools.batched(line_data[37:93], 14)
    )
    run = Run(
        seed=int(seed),
        shops=shops,
        chests=chests,
        areas=areas
    )
    return run
    

def generate_runs(filename: str) -> Generator[Run, None, None]:
    with open(filename, 'r') as f:
        lines = f.readlines()

    for line in lines:
        yield parse_line(line)

def load_config(config_path: str = "config.yaml") -> dict:
    with open(config_path, 'r') as f:
        config = yaml.safe_load(f)
    if not config: return {}

    # Map names to ID's for items, shops, and areas
    if config.get('ranges') and config.get('ranges').get('chest_ranges'):
        for ranges in config['ranges']['chest_ranges']:
            items = ranges.get('required_items', [])
            ids = list(map(lambda i: str(name_to_id[i]), items))
            ranges['required_items'] = ids

    if config.get('ranges') and config.get('ranges').get('shop_ranges'):
        for ranges in config['ranges']['shop_ranges']:
            items = ranges.get('required_upgrades', [])
            ids = list(map(lambda i: str(name_to_id[i]), items))
            ranges['required_upgrades'] = ids

    if config.get('areas'):
        for i in range(3):
            area = config['areas'][i]
            id = area_to_id[area]
            config['areas'][i] = id

    return config

def check_areas(line: list, config: dict):
    for i in range(3):
        area_id = str(config[i])
        if config['ordered']:
            if line[i+1] != area_id:
                return False
        else:
            try:
                line.index(area_id, 1, 4)
            except ValueError:
                return False
    return True

def check_ranged_requirements(line: list, ranged_config: dict) -> bool:
    if not ranged_config:
        return True
    if 'chest_ranges' in ranged_config:
        if not check_chests(line, ranged_config['chest_ranges']): return False
    if 'shop_ranges' in ranged_config:
        if not check_shops(line, ranged_config['shop_ranges']): return False
    return True

def check_chests(line: list, config: dict) -> bool:
    for range_config in config:
        start = range_config.get('start', 0)
        end = range_config.get('end', 5)
        required_items = range_config.get('required_items', [])
        unique = range_config.get('unique', False)
        
        start_index = 7 + start * 5
        end_index = 7 + end * 5 + 5
        used_chests = []
        for required_item in required_items:
            try:
                index = line.index(required_item, start_index, end_index)
            except ValueError:
                # Didn't find item
                return False
            chest_index = (index - 7) // 5
            if unique and chest_index in used_chests:
                # Items weren't in seperate chests
                return False
            used_chests.append(chest_index)
    return True

def check_shops(line: list, config: dict) -> bool:
    for range_config in config:
        start = range_config.get('start', 0)
        end = range_config.get('end', 3)
        required_upgrades = range_config.get('required_upgrades', [])
        unique = range_config.get('unique', False)
        max_cost = range_config.get('max_cost', None)
        
        start_index = 37 + start * 14
        end_index = 37 + end * 14 + 14
        used_shops = []
        for required_upgrade in required_upgrades:
            try:
                index = line.index(required_upgrade, start_index, end_index)
            except ValueError:
                # Didn't find upgrade
                return False
            if int(line[index+1]) > max_cost:
                # Price was too high
                return False
            shop_index = (index - 37) // 14
            if unique and shop_index in used_shops:
                # Upgrades weren't in seperate shops
                return False
            used_shops.append(shop_index)
    return True

def matches_criteria(line: list, config: dict) -> bool:
    if 'ranges' in config:
        if not check_ranged_requirements(line, config['ranges']):
            return False
    
    if 'areas' in config:
        if not check_areas(line, config['areas']):
            return False
    
    return True

def find_matches(filename: str, config: dict) -> list:
    matching_runs = []
    with open(filename, "r") as f:
        for line in f.readlines():
            if matches_criteria(line.split(","), config):
                matching_runs.append(line)
    return matching_runs

def full_search(config: dict):
    matching_runs = []
    for filename in os.listdir("../full_gen"):
        filename = "../full_gen/" + filename
        matches = find_matches(filename, config)
        matching_runs.extend(map(lambda x: (filename, x), matches))
    print(f"Total matching runs: {len(matching_runs)}")

    # Write csv lines
    for (filename, line) in matching_runs:
        filename = "full_search_results/" + filename.split("/")[-1]
        with open(filename, "w") as f:
            f.write(line)
            f.write("\n")

    # Pretty print runs
    with open("matching_seeds_readable.txt", 'w') as f:
        for i, (filename, line) in enumerate(matching_runs):
            unlocks = filename.split("/")[-1][:-4]
            f.write(f"Unlocks: {unlocks}\n")
            f.write(str(parse_line(line)))
            if i < len(matching_runs) - 1:
                f.write("=" * 50 + "\n\n")

def main():
    # Remove old results
    try:
        os.remove("matching_seeds.csv")
    except FileNotFoundError:
        pass
    try:
        os.remove("matching_seeds_readable.csv")
    except FileNotFoundError:
        pass
    try:
        os.makedirs("full_search_results/")
    except FileExistsError:
        pass
    for file in os.listdir("full_search_results/"):
        os.remove("full_search_results/" + file)

    config = load_config("config.yaml")
    if len(sys.argv) == 1:
        full_search(config)
        sys.exit(0)
    matching_runs = []
    matching_runs.extend(find_matches(sys.argv[1], config))
            
    print(f"Total matching runs: {len(matching_runs)}")

    # Write csv lines
    with open("matching_seeds.csv", "w") as f:
        f.writelines(matching_runs)

    # Pretty print runs
    with open("matching_seeds_readable.txt", 'w') as f:
        for i, line in enumerate(matching_runs):
            f.write(str(parse_line(line)))
            if i < len(matching_runs) - 1:
                f.write("=" * 50 + "\n\n")

if __name__ == "__main__":
    main()
