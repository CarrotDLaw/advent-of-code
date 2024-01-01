movement_dict = {"(": +1, ")": -1}
sum1 = 0
sum2 = 0
first_basement_pos = 0
file = open("src/input/AoC2015Day01.txt")
step_list = file.read()
for x in step_list:
    if x != "\n":
        sum1 += movement_dict[x]
print(sum1)
for x in step_list:
    sum2 += movement_dict[x]
    first_basement_pos += 1
    if sum2 == -1:
        break
print(first_basement_pos)
