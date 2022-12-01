if __name__ == '__main__':
    with open('input.txt') as file:
        highest_cals = [0, 0, 0]
        current_cals = 0
        for line in file:
            if line == '\n':
                if current_cals > highest_cals[0]:
                    highest_cals[2] = highest_cals[1]
                    highest_cals[1] = highest_cals[0]
                    highest_cals[0] = current_cals
                elif current_cals > highest_cals[1]:
                    highest_cals[2] = highest_cals[1]
                    highest_cals[1] = current_cals
                elif current_cals > highest_cals[2]:
                    highest_cals[2] = current_cals
                current_cals = 0
            else:
                current_cals += int(line)

        print(f'Top calories carried by elves are {highest_cals} which sums to {sum(highest_cals)}')
