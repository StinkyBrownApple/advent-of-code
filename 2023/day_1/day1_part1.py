import re

if __name__ == '__main__':
    with open('input.txt') as file:
        print(sum([int(x[0] + x[-1]) for y in file if (x := re.findall("\\d", y))]))
