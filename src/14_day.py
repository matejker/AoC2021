from copy import deepcopy


def get_polymer_rules():
    with open("14_data.txt") as fdata:
            polymers, rules_raw = fdata.read().split("\n\n")

    rules = {}
    for r in rules_raw.split("\n"):
        a, b = r.split(" -> ")
        rules[a] = b

    return polymers, rules


def generate_polymer(polymers, rules, steps=10):
    polymers_dict = {k: 0 for k in rules}

    for i in range(len(polymers) - 1):
        polymers_dict[polymers[i:i + 2]] += 1

    for _ in range(steps):
        next_polymers_dict = deepcopy({k: v for k, v in polymers_dict.items() if v > 0})
        for p, k in next_polymers_dict.items():
            polymers_dict[p[0] + rules[p]] += k
            polymers_dict[rules[p] + p[1]] += k
            polymers_dict[p] -= k

    simple_polymers_count = {}
    for p, k in polymers_dict.items():
        simple_polymers_count[p[0]] = simple_polymers_count.get(p[0], 0) + k
        simple_polymers_count[p[1]] = simple_polymers_count.get(p[1], 0) + k

    simple_polymers_count[polymers[0]] += 1
    simple_polymers_count[polymers[-1]] += 1

    result = [v // 2 for k, v in simple_polymers_count.items()]
    return max(result) - min(result)


p, r = get_polymer_rules()

print("Part 1:", generate_polymer(p, r))
print("Part 2:", generate_polymer(p, r, 40))
