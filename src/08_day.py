# cheating but I set operation in Rust are...

def decode_seven_segment(d, pattern):
    l = len(d)

    for i in range(10):
        if pattern[i] == set(list(d)):
            return i
    print(set(list(d)), pattern)
    return 0

def get_pattern(signal):
    digits = [set()] * 10

    for i in range(10): 
        for s in signal.split(" "): 
            l = len(s)
            a = set(list(s))
            if not digits[1] and l == 2: 
                digits[1] = a  # one
            elif not digits[7] and l == 3: 
                digits[7] = a  # seven
            elif not digits[4] and l == 4: 
                digits[4] = a  # four
            elif not digits[8] and l == 7: 
                digits[8] = a  # eight
            elif not digits[9] and l == 6 and len(digits[4] & a) == 4:
                digits[9] = a  # nine
            elif not digits[2] and l == 5 and len(digits[4] & a) == 2:
                digits[2] = a  # two
            elif not digits[6] and l == 6 and len(a | digits[1]) == 7:
                digits[6] = a  # six
            elif l == 6 and len(digits[7] - a) == 0 and len(digits[4] - a) == 1:
                digits[0] = a  # zero
            elif l == 5 and len(a - digits[4]) == 2 and len(digits[7] - a) == 1:
                digits[5] = a  # five
            elif l == 5 and len(a - digits[4]) == 2 and len(digits[7] - a) == 0:
                digits[3] = a  # three

    return digits


def part2():
    with open("08_data.txt") as fdata:
        lines = fdata.read().splitlines()
    total = 0
    for line in lines:
        signal, output = line.split(" | ")
        pattern = get_pattern(signal)
        parsed_output = output.split(" ")

        for i in range(len(parsed_output)):
            total += decode_seven_segment(parsed_output[i], pattern) * 10 ** (3 - i)

    return total

print(part2())