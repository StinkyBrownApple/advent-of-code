def in_range(a, b, x):
    return a <= x <= b


if __name__ == '__main__':
    total = 0
    for line in open('input.txt'):
        ranges = line.split(',')
        numsA = ranges[0].split('-')
        numsB = ranges[1].split('-')
        total += int(in_range(int(numsA[0]), int(numsA[1]), int(numsB[0])) or
                     in_range(int(numsB[0]), int(numsB[1]), int(numsA[0])))
    print(total)
