if __name__ == '__main__':
    scores_dict = {
        'X': 1,  # Rock
        'Y': 2,  # Paper
        'Z': 3,  # Scissors
        'A X': 3,
        'A Y': 6,
        'A Z': 0,
        'B X': 0,
        'B Y': 3,
        'B Z': 6,
        'C X': 6,
        'C Y': 0,
        'C Z': 3
    }
    score = 0
    with open('input.txt') as file:
        for line in file:
            score += scores_dict[line.strip()[-1]] + scores_dict[line.strip()]
    print(score)
