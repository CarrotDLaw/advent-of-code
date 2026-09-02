def is_repeated_twice(s: str) -> bool:
    return s[: (len(s) // 2)] == s[(len(s) // 2) :]


def is_repeated_n_times(s: str, n: int) -> bool:
    if len(s) % n != 0:
        return False

    k: int = len(s) // n
    block: str = s[:k]
    for i in range(k, len(s), k):
        if s[i : i + k] != block:
            return False

    return True


all_ids: list[str] = []

with open("src/input/AoC2025Day02.txt") as file:
    strings = file.readline().strip().split(",")
    for s in strings:
        start, end = map(int, s.split("-"))
        all_ids.extend([str(i) for i in range(start, end + 1)])

sum: int = 0
for s in all_ids:
    if is_repeated_twice(s):
        sum += int(s)

print(sum)

sum: int = 0
for s in all_ids:
    for n in range(2, len(s) + 1):
        if is_repeated_n_times(s, n):
            sum += int(s)
            break

print(sum)
