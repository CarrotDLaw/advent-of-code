strings = []

with open("src/input/AoC2023Day01.txt") as file:
    strings = [entry.strip() for entry in file.readlines()]


def sum_l_r_digits(string: str) -> int:
    l_digit = None
    r_digit = None
    for c in string:
        if c.isdigit():
            if l_digit is None:
                l_digit = int(c)
            r_digit = int(c)

    # print(f"{l_digit} {r_digit}")
    return l_digit * 10 + r_digit


def part_1(strings: list[str]) -> int:
    return sum([sum_l_r_digits(string) for string in strings])


def part_2(strings: list[str]) -> int:
    sum = 0
    spelt_numbers = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ]
    for string in strings:
        nums: list[int] = []
        for i, c in enumerate(string):
            for v, n in enumerate(spelt_numbers):
                if n in string[i : i + len(n)]:
                    nums.append(int(v))
            if c.isdigit():
                nums.append(int(c))
        sum += nums[0] * 10 + nums[-1]
    return sum


print(part_1(strings))
print(part_2(strings))
