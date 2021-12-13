import matplotlib.pyplot as plt
from copy import deepcopy


def get_folds_and_dots():
    with open("13_data.txt") as fdata:
            dots, folds = fdata.read().split("\n\n")

    dots_set = set()
    for d in dots.split("\n"):
        i, j = d.split(",")
        dots_set.add((int(i), int(j)))

    folds_list = []
    for f in folds.split("\n"):
        folds_list.append(f[len("fold along "):])

    return dots_set, folds_list


def fold(dots, folds):
    for f in folds:
        temp_dots = deepcopy(dots)
        v = int(f[2:])
        for d in dots:
            if f[0] == "x":
                if d[0] == v:
                   temp_dots.remove(d)
                elif d[0] == 2 * v:
                    temp_dots.remove(d)
                    temp_dots.add((0, d[1]))
                elif d[0] > v:
                   temp_dots.remove(d)
                   temp_dots.add((int(d[0] - 2 * (d[0] % v)), d[1]))
            elif f[0] == "y":
                if d[1] == v:
                   temp_dots.remove(d)
                elif d[1] == 2 * v:
                    temp_dots.remove(d)
                    temp_dots.add((d[0], 0))
                elif d[1] > v:
                   temp_dots.remove(d)
                   temp_dots.add((d[0], int(d[1] - 2 * (d[1] % v))))

        dots = deepcopy(temp_dots)

    return len(dots), dots


dots, folds = get_folds_and_dots()
nn, _ = fold(dots, [folds[0]])
print("Part1: ", nn)
n, d = fold(dots, folds)
print("Part2:", n)

plt.figure(figsize=(20, 5))
plt.axis('off')
plt.scatter([x for x, _ in d], [-y for _, y in d], marker="s", s=100)
plt.savefig("day13.png")
