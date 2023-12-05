import re

if __name__ == '__main__':
    with open('input.txt') as file:
        def min_cubes(g, colour):
            return max([int(x) for x in re.findall("(\\d{1,2}) " + colour, g)])
        print(sum([min_cubes(line, "red") * min_cubes(line, "green") * min_cubes(line, "blue") for line in file]))


