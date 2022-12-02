if __name__ == '__main__':
    choice_score_dict = {
        'X': 1,  # Rock
        'Y': 2,  # Paper
        'Z': 3,  # Scissors
    }
    outcome_score_dict = {
        'X': 0,  # Lose
        'Y': 3,  # Draw
        'Z': 6,  # Win
    }
    choice_dict = {
        'A X': 'Z',
        'A Y': 'X',
        'A Z': 'Y',
        'B X': 'X',
        'B Y': 'Y',
        'B Z': 'Z',
        'C X': 'Y',
        'C Y': 'Z',
        'C Z': 'X',
    }
    score = 0
    with open('input.txt') as file:
        for line in file:
            score += choice_score_dict[choice_dict[line.strip()]] + outcome_score_dict[line.strip()[-1]]
    print(score)
