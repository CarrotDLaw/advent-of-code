elves = []

with open("src/input/AoC2022Day01.txt") as file:
    elf = 0
    for line in file.readlines():
        if line != '\n':
            elf += int(line)
        else:
            elves.append(elf)
            elf = 0

elves.sort()

print(elves[-1])  # part 1
print(sum(elves[-3:]))  # part 2
