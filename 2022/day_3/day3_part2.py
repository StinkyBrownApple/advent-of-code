def split_input(inp):
    half = int(len(inp)/2)
    return inp[:half], inp[half:]


def conv_to_binary(char):
    return 1 << (ord(char)-(96 if char.islower() else 38))


def get_binary_rep(chars):
    binary = 0b0
    for char in chars:
        binary = binary | conv_to_binary(char)
    return binary


def prio_from_binary(binary):
    return binary.bit_length() - 1


if __name__ == '__main__':
    total = 0
    lines = []
    for line in open('input.txt'):
        lines.append(line.strip())
        if len(lines) == 3:
            dupe = get_binary_rep(lines.pop()) & get_binary_rep(lines.pop()) & get_binary_rep(lines.pop())
            total += prio_from_binary(dupe)
    print(total)
