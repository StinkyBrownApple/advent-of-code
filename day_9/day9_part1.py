def move_tail(head_x, head_y, tail_x, tail_y):


def main():
    head_x, head_y, tail_x, tail_y = 0, 0, 0, 0
    for line in open('input.txt'):
        instruction = line.strip().split(' ')
        move = instruction[0]
        amount = int(instruction[1])
        if move == 'R':
            for _ in range(amount):
                head_x += 1
                tail_x, tail_y = move_tail(head_x, head_y, tail_x, tail_y)



if __name__ == '__main__':
    main()