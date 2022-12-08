def scenic_score(file, i, j, width, height):  # Can be tidied up by removing code duplication
    tree_height = int(file[j][i])
    if i == 0 or i == width - 1 or j == 0 or j == height - 1:
        return 0
    # Right check
    right_vis = 0
    for x in range(i + 1, width):
        right_vis += 1
        if int(file[j][x]) >= tree_height:
            break
    # Left check
    left_vis = 0
    for x in range(i - 1, -1, -1):
        left_vis += 1
        if int(file[j][x]) >= tree_height:
            break
    # Down check
    down_vis = 0
    for y in range(j + 1, height):
        down_vis += 1
        if int(file[y][i]) >= tree_height:
            break
    # Up check
    up_vis = 0
    for y in range(j - 1, -1, -1):
        up_vis += 1
        if int(file[y][i]) >= tree_height:
            break
    return up_vis * left_vis * right_vis * down_vis


def main():
    file = open('input.txt').readlines()
    file = [line.strip() for line in file]
    width = len(file[0])
    height = len(file)
    current_highest = 0
    for j in range(height):
        for i in range(width):
            score = scenic_score(file, i, j, width, height)
            if score > current_highest:
                current_highest = score
    print(current_highest)


if __name__ == '__main__':
    main()
