shape_score = {'A': 1, 'B': 2, 'C': 3, 'X': 1, 'Y': 2, 'Z': 3}
scores1 = []
scores2 = []

with open("src/input/AoC2022Day02.txt") as file:
    for line in file.readlines():
        score1 = 0
        score2 = 0
        p1 = line[0]
        p2 = line[2]
        if p1 == 'A':  # part 1
            if p2 == 'Y':
                score1 += 6
            elif p2 == 'X':
                score1 += 3
        elif p1 == 'B':
            if p2 == 'Z':
                score1 += 6
            elif p2 == 'Y':
                score1 += 3
        elif p1 == 'C':
            if p2 == 'X':
                score1 += 6
            elif p2 == 'Z':
                score1 += 3
        score1 += shape_score[p2]
        scores1.append(score1)

        if p2 == 'X':  # part 2  # lose
            if p1 == 'A':
                score2 += 3
            elif p1 == 'B':
                score2 += 1
            elif p1 == 'C':
                score2 += 2
        elif p2 == 'Y':  # draw
            score2 += 3
            score2 += shape_score[p1]
        elif p2 == 'Z':  # win
            score2 += 6
            if p1 == 'A':
                score2 += 2
            elif p1 == 'B':
                score2 += 3
            elif p1 == 'C':
                score2 += 1
        scores2.append(score2)

print(sum(scores1))
print(sum(scores2))
