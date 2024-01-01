def part_one(data) -> int:
    data = [set(group.replace("\n", "")) for group in data]

    return sum([len(group) for group in data])


def part_two(data) -> int:
    count = 0

    for group in data:
        persons = group.splitlines()
        answers = set(persons[0])

        for person in persons:
            answers &= set(person)

        count += len(answers)

    return count


with open("src/input/AoC2020Day06.txt") as file:
    input = file.read().strip().split("\n\n")

print(part_one(input))
print(part_two(input))
