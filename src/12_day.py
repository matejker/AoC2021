from copy import deepcopy


def get_graph():
    graph = {}
    with open("12_data.txt") as fdata:
        links = fdata.read().splitlines()

    for link in links:
        a, b = link.split("-")
        graph[a] = graph.get(a, set()) | {b}
        graph[b] = graph.get(b, set()) | {a}

    return graph


def get_path(graph, cave, small_caves, twice=False):
    if cave == "end":
        return 1

    if cave in small_caves:
        if not twice or cave == "start":
            return 0
        else:
            twice = False

    if cave.islower():
        small_caves |= {cave}

    return sum([get_path(graph, neighbour, deepcopy(small_caves), twice) for neighbour in graph[cave]])


def part1():
    graph = get_graph()
    print(graph)
    print(f"Part 1: {get_path(graph, 'start', set())}")


def part2():
    graph = get_graph()
    print(f"Part 2: {get_path(graph, 'start', set(), True)}")

part1()
part2()