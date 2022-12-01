if __name__ == '__main__':
    with open('input.txt') as file:
        highest_cals = 0
        current_cals = 0
        for line in file:
            if line == '\n':
                highest_cals = current_cals if current_cals > highest_cals else highest_cals
                current_cals = 0
            else:
                current_cals += int(line)

        print(f'Most calories carried by an elf is {highest_cals}')
