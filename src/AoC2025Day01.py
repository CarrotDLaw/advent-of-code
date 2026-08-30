DIAL_SIZE: int = 100
dir_sign_map: dict[str, int] = {"L": -1, "R": 1}

dial_pos: int = 50
count: int = 0
strings: list[str]
with open("src/input/AoC2025Day01.txt") as file:
    strings = [entry.strip() for entry in file.readlines()]

for s in strings:
    dir: str = s[0]
    dist: int = int(s[1:])

    dial_pos = (dial_pos + dir_sign_map.get(dir, 0) * dist + DIAL_SIZE) % DIAL_SIZE

    if dial_pos == 0:
        count += 1

print(count)

dial_pos: int = 50
count = 0
for s in strings:
    dir: str = s[0]
    dist: int = int(s[1:])
    next_zero_dist: int = 0

    if dir == "R":
        next_zero_dist = DIAL_SIZE - dial_pos

    if dir == "L":
        next_zero_dist = dial_pos
        if next_zero_dist == 0:
            next_zero_dist = DIAL_SIZE

    if dist >= next_zero_dist:
        count += 1 + (dist - next_zero_dist) // DIAL_SIZE

    dial_pos = (dial_pos + dir_sign_map.get(dir, 0) * dist + DIAL_SIZE) % DIAL_SIZE

print(count)
