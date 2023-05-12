file = open("src/input/AoC2015Day02.txt")
prism_list = []
c = 0
sq_ft_paper = 0
ft_ribbon = 0
for x in file:
    prism_list.append(x)
    int_list = []
    for i in prism_list[c].split("x"):
        int_list.append(int(i))
    prism_list[c] = int_list
    prism_list[c].sort()
    c += 1
for i in prism_list:
    sq_ft_paper += 2 * (i[0] * i[1] + i[0] * i[2] + i[1] * i[2]) + i[0] * i[1]
for i in prism_list:
    ft_ribbon += 2 * (i[0] + i[1]) + i[0] * i[1] * i[2]
print(sq_ft_paper)
print(ft_ribbon)
