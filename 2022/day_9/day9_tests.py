from day9_part1 import move_tail


def print_grid(head_x, head_y, tail_x, tail_y):
    grid = []
    for j in range(6):
        line = ''
        for i in range(6):
            if i is head_x and j is head_y:
                line += 'H'
            elif i is tail_x and j is tail_y:
                line += 'T'
            else:
                line += '.'
        grid.insert(0, line)
    [print(l) for l in grid]
    

def test_move_tail():
    head_x, head_y = 0, 1
    tail_x, tail_y = 2, 0
    print_grid(head_x, head_y, tail_x, tail_y)
    tail = move_tail(head_x, head_y, tail_x, tail_y)
    print_grid(head_x, head_y, tail[0], tail[1])


if __name__ == '__main__':
    test_move_tail()