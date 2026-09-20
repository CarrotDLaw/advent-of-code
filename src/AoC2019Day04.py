l = 231832
u = 767346


def check_increase(num: int) -> bool:
    s = str(num)
    for i in range(len(s) - 1):
        if s[i] > s[i + 1]:
            return False

    return True


def check_doubles(num: int) -> bool:
    s = str(num)
    for i in range(len(s) - 1):
        if s[i] == s[i + 1]:
            return True

    return False


def check_part_2(num: int) -> bool:
    s = str(num)

    if s[0] == s[1] and s[1] != s[2]:
        return True
    if s[0] != s[1] and s[1] == s[2] and s[2] != s[3]:
        return True
    if s[1] != s[2] and s[2] == s[3] and s[3] != s[4]:
        return True
    if s[2] != s[3] and s[3] == s[4] and s[4] != s[5]:
        return True
    if s[3] != s[4] and s[4] == s[5]:
        return True

    return False


check_increase_list: list[int] = [n for n in range(l, u + 1) if check_increase(n)]
check_doubles_list: list[int] = [n for n in check_increase_list if check_doubles(n)]
part_2_list: list[int] = [n for n in check_doubles_list if check_part_2(n)]

print(len(check_doubles_list))
print(len(part_2_list))
