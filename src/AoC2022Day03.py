first_halves = []
second_halves = []
part2_list = []
sum1, sum2 = 0, 0

with open("src/input/AoC2022Day03.txt") as file:
    for line in file.readlines():
        l = len(line) // 2
        part2_list.append(list(line.strip()))
        first_halves.append(list(line[:l]))
        second_halves.append(list(line[l:]))

for i in range(len(first_halves)):  # part 1
    (result,) = set(first_halves[i]) & set(second_halves[i])
    str(result)
    if result.islower():
        sum1 += ord(result) - 96
    elif result.isupper():
        sum1 += ord(result) - 64 + 26

for i in range(0, len(part2_list) - 2, 3):  # part 2
    (result,) = set(part2_list[i]) & set(part2_list[i + 1]) & set(part2_list[i + 2])
    str(result)
    if result.islower():
        sum2 += ord(result) - 96
    elif result.isupper():
        sum2 += ord(result) - 64 + 26

print(sum1)
print(sum2)
