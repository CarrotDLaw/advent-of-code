def house_cal(movement_input) -> tuple:
    x = 0
    y = 0
    path = {(0, 0)}
    for needle in movement_input:
        if needle == "^" or needle == "v":
            y += compass[needle]
        if needle == ">" or needle == "<":
            x += compass[needle]
        path.add((x, y))
    return path


file = open("src/input/AoC2015Day03.txt")
movement = file.read()
compass = {"^": +1, ">": +1, "v": -1, "<": -1}
print(len(house_cal(movement)))
even = True
santa_movement = ""
robot_movement = ""
for i in movement:
    if even:
        santa_movement += i
    else:
        robot_movement += i
    even = not even
santa_house = house_cal(santa_movement)
robot_house = house_cal(robot_movement)
santa_house.update(robot_house)
print(len(santa_house))
