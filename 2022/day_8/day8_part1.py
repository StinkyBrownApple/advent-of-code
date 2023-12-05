def is_visible(file, i, j, width, height):  # Can be improved by adding early exit when checking rows and columns
    tree_height = int(file[j][i])           # and tidied up by removing code duplication
    left_blocked = False
    right_blocked = False
    up_blocked = False
    down_blocked = False
    if i == 0 or i == width - 1 or j == 0 or j == height - 1:
        return True
    for x in range(width):
        if x == i:
            if not left_blocked:
                return True
            else:
                continue
        if int(file[j][x]) >= tree_height:
            if x < i:
                left_blocked = True
            else:
                right_blocked = True
    if not right_blocked:
        return True
    for y in range(height):
        if y == j:
            if not up_blocked:
                return True
            else:
                continue
        if int(file[y][i]) >= tree_height:
            if y < j:
                up_blocked = True
            else:
                down_blocked = True
    if not down_blocked:
        return True
    return False


def main():
    file = open('input.txt').readlines()
    file = [line.strip() for line in file]
    width = len(file[0])
    height = len(file)
    total = 0
    for j in range(height):
        for i in range(width):
            total += int(is_visible(file, i, j, width, height))
    print(total)


if __name__ == '__main__':
    main()
