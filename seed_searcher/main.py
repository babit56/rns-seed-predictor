import functools
import itertools
import re
import sys
from dataclasses import dataclass
from multiprocessing import Pool
from pathlib import Path
from typing import Generator

import yaml


@dataclass
class Chest:
    contents: list[str]


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
        lines = [f"Seed {self.seed}", f"Areas: {self.areas}", ""]

        # Print chests
        for i, chest in enumerate(self.chests, 1):
            lines.append(f"CHEST {i}")
            for item in chest.contents:
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

with open("ids.txt", "r") as f:
    for line in f:
        match = re.match(r'\[(\d+)\s+"(.+)"\]', line)
        if match:
            id = int(match.group(1))
            name = match.group(2)
            id_to_name[id] = name
            name_to_id[name] = id

# Homemade ID's
id_area_list = [
    (0, "hw_outskirts"),
    (1, "hw_nest"),
    (2, "hw_arsenal"),
    (3, "hw_lighthouse"),
    (4, "hw_streets"),
    (5, "hw_lakeside"),
    (6, "hw_keep"),
    (7, "hw_geode"),
    (8, "hw_sanct"),
    (9, "hw_depths"),
    (10, "hw_aurum"),
    (11, "hw_darkhall"),
]
id_to_area: dict[int, str] = {}
area_to_id: dict[str, int] = {}

for id, name in id_area_list:
    id_to_area[id] = name
    area_to_id[name] = id

# Indicies
# end index shouldbe index of first item after
area_start = 2
area_end = area_start + 3
item_start = 6
chest_size = 5
item_end = item_start + chest_size * 6
shop_start = 36
shop_length = 14
shop_end = shop_start + shop_length * 4

def parse_line(line: str) -> Run:
    line_data = line.strip().split(",")
    seed = line_data[0]
    areas = tuple(map(lambda id: id_to_area[int(id)], line_data[area_start:area_end]))
    chests = tuple(
        Chest(contents=[id_to_name[int(item_id)] for item_id in chest_data])
        for chest_data in itertools.batched(line_data[item_start:item_end], chest_size)
    )
    shops = tuple(
        Shop(
            upgrades=tuple(
                Upgrade(gem=id_to_name[int(upgrade_data[0])], cost=int(upgrade_data[1]))
                for upgrade_data in itertools.batched(shop_data[6:], 2)
            ),
            potions=tuple(
                Potion(name=id_to_name[int(name)], cost=int(cost))
                for name, cost in zip(shop_data[:3], shop_data[3:6])
            ),
        )
        for shop_data in itertools.batched(line_data[shop_start:shop_end], shop_length)
    )
    run = Run(
        seed=int(seed),
        shops=shops,
        chests=chests,
        areas=areas,
    )
    return run


def generate_runs(filename: str) -> Generator[Run, None, None]:
    with open(filename, "r") as f:
        lines = f.readlines()

    for line in lines:
        yield parse_line(line)


def load_config(config_path: str = "config.yaml") -> dict:
    with open(config_path, "r") as f:
        config = yaml.safe_load(f)
    if not config:
        return {}

    # Map names to ID's for items, shops, and areas
    if config.get("ranges") and config.get("ranges").get("chest_ranges"):
        for ranges in config["ranges"]["chest_ranges"]:
            items = ranges.get("required_items", [])
            ids = list(map(lambda i: str(name_to_id[i]), items))
            ranges["required_items"] = ids

    if config.get("ranges") and config.get("ranges").get("shop_ranges"):
        for ranges in config["ranges"]["shop_ranges"]:
            items = ranges.get("required_upgrades", [])
            ids = list(map(lambda i: str(name_to_id[i]), items))
            ranges["required_upgrades"] = ids

    if config.get("areas"):
        for i in range(3):
            area = config["areas"][i]
            id = area_to_id[area]
            config["areas"][i] = id

    return config


def check_areas(line: list, config: dict):
    for i in range(3):
        area_id = str(config[i])
        if config["ordered"]:
            if line[i + area_start] != area_id:
                return False
        else:
            try:
                line.index(area_id, area_start, area_end)
            except ValueError:
                return False
    return True


def check_ranged_requirements(line: list, ranged_config: dict) -> bool:
    if not ranged_config:
        return True
    if "chest_ranges" in ranged_config and ranged_config["chest_ranges"]:
        if not check_chests(line, ranged_config["chest_ranges"]):
            return False
    if "shop_ranges" in ranged_config and ranged_config["shop_ranges"]:
        if not check_shops(line, ranged_config["shop_ranges"]):
            return False
    return True


def check_chests(line: list, config: dict) -> bool:
    for range_config in config:
        start_chest = range_config.get("start", 0)
        end_chest = range_config.get("end", 5)
        required_items = range_config.get("required_items", [])
        unique = range_config.get("unique", False)
        assert 0 <= start_chest <= end_chest < 6, f"chest start/end not within bounds, got: 0 <= {start_chest} <= {end_chest} < 6"

        start_index = item_start + start_chest * chest_size
        end_index = item_start + end_chest * chest_size + chest_size
        used_chests = []
        for required_item in required_items:
            try:
                index = line.index(required_item, start_index, end_index)
            except ValueError:
                # Didn't find item
                return False
            chest_index = (index - item_start) // chest_size
            if unique and chest_index in used_chests:
                # Items weren't in seperate chests
                return False
            used_chests.append(chest_index)
    return True


def check_shops(line: list, config: dict) -> bool:
    for range_config in config:
        start = range_config.get("start", 0)
        end = range_config.get("end", 3)
        required_upgrades = range_config.get("required_upgrades", [])
        unique = range_config.get("unique", False)
        max_cost = range_config.get("max_cost", None)
        assert 0 <= start <= end < 4, f"shop start/end not within bounds, got: 0 <= {start} <= {end} < 4"
        assert 23 <= max_cost <= 27, "max_cost not within bounds 23-27"

        start_index = shop_start + start * shop_length
        end_index = shop_start + end * shop_length + shop_length
        used_shops = []
        for required_upgrade in required_upgrades:
            try:
                index = line.index(required_upgrade, start_index, end_index)
            except ValueError:
                # Didn't find upgrade
                return False
            if int(line[index + 1]) > max_cost:
                # Price was too high
                return False
            shop_index = (index - shop_start) // shop_length
            if unique and shop_index in used_shops:
                # Upgrades weren't in seperate shops
                return False
            used_shops.append(shop_index)
    return True


def matches_criteria(line: list, config: dict) -> bool:
    if "ranges" in config:
        if not check_ranged_requirements(line, config["ranges"]):
            return False

    if "areas" in config:
        if not check_areas(line, config["areas"]):
            return False

    return True


def find_matches(filename: Path, config: dict) -> list:
    matching_runs = []
    with open(filename, "r") as f:
        for line in f.readlines():
            if matches_criteria(line.split(","), config):
                matching_runs.append(line)
    return matching_runs


def inner_search(path: Path, config: dict) -> tuple[Path, list]:
    return (path, find_matches(path, config))


def full_search(dir: Path, config: dict):
    find = functools.partial(inner_search, config=config)
    with Pool() as p:
        # paths = map(lambda path: (path, config), Path("../full_gen").iterdir())
        matching_runs = list(p.map(find, dir.iterdir()))

    total_matches = 0
    # Write csv lines
    for path, matches in matching_runs:
        path = Path("full_search_results/") / path.name
        total_matches += len(matches)
        if len(matches) > 0:
            with open(path, "w") as f:
                f.writelines(matches)

    # Pretty print runs
    with open("matching_seeds_readable.txt", "w") as f:
        for path, matches in matching_runs:
            unlocks = path.stem
            for i, line in enumerate(matches):
                f.write(f"Unlocks: {unlocks}\n")
                f.write(str(parse_line(line)))
                f.write("=" * 50 + "\n")
    print(f"Total matching runs: {total_matches}")


def prepare_filesystem():
    dir = Path(".")
    for file in dir.glob("matching*"):
        file.unlink()
    # try:
    #     os.remove("matching_seeds.csv")
    # except FileNotFoundError:
    #     pass
    # try:
    #     os.remove("matching_seeds_readable.csv")
    # except FileNotFoundError:
    #     pass
    results_dir = dir / "full_search_results"
    results_dir.mkdir(exist_ok=True)
    for file in results_dir.iterdir():
        file.unlink()


def main():
    prepare_filesystem()
    config = load_config("config.yaml")
    if len(sys.argv) == 1:
        full_search(Path("../full_gen"), config)
        sys.exit(0)
    path = Path(sys.argv[1])
    if path.is_dir():
        full_search(path, config)
        sys.exit(0)
    matching_runs = []
    matching_runs.extend(find_matches(path, config))

    print(f"Total matching runs: {len(matching_runs)}")

    # Write csv lines
    with open("matching_seeds.csv", "w") as f:
        f.writelines(matching_runs)

    # Pretty print runs
    with open("matching_seeds_readable.txt", "w") as f:
        for i, line in enumerate(matching_runs):
            f.write(str(parse_line(line)))
            if i < len(matching_runs) - 1:
                f.write("=" * 50 + "\n\n")


if __name__ == "__main__":
    main()
