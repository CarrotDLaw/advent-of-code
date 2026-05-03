arr_1 = []
arr_2 = []

with open("src/input/AoC2024Day01.txt") as file:
    strings = [entry.strip() for entry in file.readlines()]
    for s in strings:
        arr_1.append(int(s.split()[0]))
        arr_2.append(int(s.split()[1]))

arr_1.sort()
arr_2.sort()

# part 1
totalA_distance = 0
for i in range(len(arr_1)):
    totalA_distance += abs(arr_1[i] - arr_2[i])

print(totalA_distance)

# part 2

dict_1 = {}
dict_2 = {}

for key in arr_1:
    dict_1[key] = dict_1.get(key, 0) + 1

for key in arr_2:
    dict_2[key] = dict_2.get(key, 0) + 1

similarity_score = 0
for key in dict_1:
    similarity_score += key * dict_1.get(key, 0) * dict_2.get(key, 0)

print(similarity_score)

