#!/usr/bin/env python3
import hashlib
import json
from itertools import permutations

BASE = {"hour": 7, "minute": 30, "days": [1, 2, 3, 4, 5]}


def digest(value):
    return hashlib.sha256(
        json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()
    ).hexdigest()


baseline = digest(BASE)
orders = 0
for order in permutations(tuple(BASE)):
    reordered = {key: BASE[key] for key in order}
    assert digest(reordered) == baseline, f"schedule key order changed identity: {order}"
    orders += 1
assert orders == 6

for mutated in (
    {**BASE, "hour": 8},
    {**BASE, "minute": 31},
    {**BASE, "days": [1, 2, 3, 4]},
    {**BASE, "days": [1, 2, 3, 4, 5, 6]},
):
    assert digest(mutated) != baseline, f"schedule mutation preserved identity: {mutated}"

assert digest({"hour": 7, "minute": 30}) != baseline
assert digest({**BASE, "timezone": "America/Lima"}) != baseline
print("alarm schedule semantic digest permutation/mutation proof: ok")
