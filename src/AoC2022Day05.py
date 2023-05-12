import re

file_str = []
stacks1 = []
stacks2 = []
commands = []

for i in range(9):
    stacks1.append([])
    stacks2.append([])

with open("src/input/AoC2022Day05.txt") as file:
    for line in file.readlines():
        file_str.append(line)

    line_num = 0
    for i in file_str:  # init stacks
        line_num += 1
        if i == '\n':
            for j in stacks1:
                j.pop()
                j.reverse()
            for j in stacks2:
                j.pop()
                j.reverse()
            break
        for j, k in zip(range(1, 36, 4), range(9)):
            if i[j] != ' ':
                stacks1[k].append(i[j])
                stacks2[k].append(i[j])

    for i in file_str[line_num:]:  # init commands
        commands.append([int(x) for x in re.findall(r'\d+', i)])

for i in commands:  # part 1
    for j in range(i[0]):
        stacks1[i[2] - 1].append(stacks1[i[1] - 1].pop())

for i in commands:  # part 2
    nl = []
    for j in range(i[0]):
        nl.append(stacks2[i[1] - 1].pop())
    for j in range(i[0]):
        stacks2[i[2] - 1].append(nl.pop())

msg1 = ""
msg2 = ""
for i in stacks1:
    msg1 += i[-1]
for i in stacks2:
    msg2 += i[-1]
print(msg1)
print(msg2)
