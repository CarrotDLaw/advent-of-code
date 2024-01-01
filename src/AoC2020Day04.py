import re


def validate_byr(field) -> bool:
    return re.match(r"^(19[2-9][0-9]|200[0-2])$", field)


def validate_ecl(field) -> bool:
    return re.match(r"^(amb|blu|brn|gry|grn|hzl|oth)$", field)


def validate_eyr(field) -> bool:
    return re.match(r"^(202[0-9]|2030)$", field)


def validate_hcl(field) -> bool:
    return re.match(r"^(#[0-9a-f]{6})$", field)


def validate_hgt(field) -> bool:
    return re.match(r"^((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$", field)


def validate_iyr(field) -> bool:
    return re.match(r"^(201[0-9]|2020)$", field)


def validate_pid(field) -> bool:
    return re.match(r"^[0-9]{9}$", field)


required_fields = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"]

with open("src/input/AoC2020Day04.txt") as file:
    entries = file.read().strip().split("\n\n")
    passports = []
    for entry in entries:
        passport = {}
        for fields in entry.replace("\n", " ").split(" "):
            i, j = fields.split(":")
            passport[i] = j
        passports.append(passport)

valid_passports1 = 0
valid_passports2 = 0

for passport in passports:
    if all(keys in passport for keys in required_fields):
        valid_passports1 += 1
        if (
            validate_byr(passport["byr"])
            and validate_ecl(passport["ecl"])
            and validate_eyr(passport["eyr"])
            and validate_hcl(passport["hcl"])
            and validate_hgt(passport["hgt"])
            and validate_iyr(passport["iyr"])
            and validate_pid(passport["pid"])
        ):
            valid_passports2 += 1

print(valid_passports1)
print(valid_passports2)
