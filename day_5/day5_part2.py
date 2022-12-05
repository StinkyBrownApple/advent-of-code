import re


def parse_start(lines_list):
    starting_positions = [[] for _ in range(9)]
    for i in range(len(lines_list)):
        for j in range(0, 9):
            char = lines_list[-1-i][(j * 4) + 1]
            if char != ' ':
                starting_positions[j].append(char)
    return starting_positions


def move_stacks(stacks, instructions):
    stacks[instructions[2]-1].extend(stacks[instructions[1]-1][-instructions[0]:])
    stacks[instructions[1]-1] = stacks[instructions[1]-1][:-instructions[0]]


def main():
    init_positions = []
    stacks = None
    output = ''
    for line in open('input.txt'):
        if stacks is None:
            if line.strip() == '':
                init_positions.pop()
                stacks = parse_start(init_positions)
            else:
                init_positions.append(line.strip())
        else:
            instructions = [int(x) for x in re.findall('\\d{1,2}', line)]
            move_stacks(stacks, instructions)
    for stack in stacks:
        output += stack[-1]
    print(output)


if __name__ == '__main__':
    main()
