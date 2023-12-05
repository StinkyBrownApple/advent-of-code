import re

if __name__ == '__main__':
    limits = {
        "red": 12,
        "green": 13,
        "blue": 14
    }
    with open('input.txt') as file:
        def possible(g):
            return len(
                [x for x in re.findall("(\\d{1,2}) (red|green|blue)", g) if int(x[0]) > limits[x[1]]]
            ) == 0
        print(sum([int(line.split(": ")[0].split()[1]) for line in file if possible(line)]))


