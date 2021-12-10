def part2():
    with open("10_data.txt") as fdata:
        lines = fdata.read().splitlines()

    opening = list("([{<")
    closing = list(")]}>")

    scores = []

    for l in lines:
        heap = []
        score = 0

        for e in list(l):
            if e in opening:
                heap.append(e)
            elif e in closing:
                i = closing.index(e)
                if opening[i] != heap.pop():
                    heap = []
                    break

        if not heap:
            continue

        for h in heap[::-1]:
            cost = opening.index(h) + 1
            score = 5 * score + cost
        scores.append(score)

    scores.sort()

    print(scores[len(scores) // 2])

part2()
