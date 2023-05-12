l = []
sum1 = 0
sum2 = 0

with open("src/input/AoC2022Day04.txt") as file:
    for line in file.readlines():
        line = line.strip().split(",")
        s_0 = int(line[0][:line[0].find("-")])
        e_0 = int(line[0][line[0].find("-") + 1:])
        s_1 = int(line[1][:line[1].find("-")])
        e_1 = int(line[1][line[1].find("-") + 1:])
        l.append([s_0, e_0, s_1, e_1])

for i in l:  # part 1
    if (i[0] <= i[2] and i[1] >= i[3]) or (i[0] >= i[2] and i[1] <= i[3]):
        sum1 += 1

for i in l:  # part 2
    if (i[0] >= i[2] and i[0] <= i[3]) or (i[1] >= i[2] and i[1] <= i[3]) or (
            i[2] >= i[0] and i[2] <= i[1]) or (i[3] >= i[0] and i[3] <= i[1]):
        sum2 += 1

print(sum1)
print(sum2)
