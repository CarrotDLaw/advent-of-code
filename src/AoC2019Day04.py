lower = 231832
upper = 767346


def check_increase(num) -> bool:
    a = str(num)
    for i in range(5):
        if a[i] > a[i + 1]:
            return False

    return True


def check_doubles(num) -> bool:
    a = str(num)
    for i in range(5):
        if a[i] == a[i + 1]:
            return True

    return False


def part2(num) -> bool:
    a = str(num)
    valid = False
    if a[0] == a[1] and a[1] != a[2]:
        valid = True
    if a[1] == a[2] and a[2] != a[3]:
        valid = True
    if a[2] == a[3] and a[3] != a[4]:
        valid = True
    if a[3] == a[4] and a[4] != a[5]:
        valid = True
    if a[4] == a[5] and a[4] != a[3]:
        valid = True
    return valid


check_increase_list = [
    num for num in range(lower, upper + 1) if check_increase(num)
]
check_doubles_list = [num for num in check_increase_list if check_doubles(num)]
part2_list = [num for num in check_doubles_list if part2(num)]

print(len(check_doubles_list))
print(len(part2_list))
