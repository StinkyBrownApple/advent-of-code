import math

def clamp(val, smallest, biggest):
    return max(smallest, min(val, biggest))


def move_tail(head_x, head_y, tail_x, tail_y):
    diff_x = (head_x - tail_x)
    diff_y = (head_y - tail_y)
    if math.sqrt(diff_x ** 2 + diff_y ** 2) > 1.5:
        return (tail_x + clamp(diff_x, -1, 1), tail_y + clamp(diff_y, -1, 1))
    return (tail_x, tail_y)


def main():
    rope = [(0, 0) for _ in range(10)]
    move_dict = {'R': (1, 0), 'L': (-1, 0), 'U': (0, 1), 'D': (0, -1)}
    tail_positions = {(0, 0)}
    for line in open('input.txt'):
        instruction = line.strip().split(' ')
        move = instruction[0]
        amount = int(instruction[1])
        for _ in range(amount):
            rope[0] = (rope[0][0] + move_dict[move][0], rope[0][1] + move_dict[move][1])
            for i in range(len(rope) - 1):
                rope[i+1] = move_tail(rope[i][0], rope[i][1], rope[i+1][0],  rope[i+1][1])
            tail_positions.add(rope[i+1])
    print(len(tail_positions))



if __name__ == '__main__':
    main()