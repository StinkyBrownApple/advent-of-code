import re

if __name__ == '__main__':
    with open('input.txt') as file:
        regex = "(?=(\\d|one|two|three|four|five|six|seven|eight|nine))"
        lookup_dict = {"one": "1", "two": "2", "three": "3", "four": "4", "five": "5", "six": "6", "seven": "7", "eight": "8", "nine": "9"}
        def lookup(s): return lookup_dict[s] if s in lookup_dict else s
        print(sum([int(lookup(x[0]) + lookup(x[-1])) for y in file if (x := re.findall(regex, y))]))
