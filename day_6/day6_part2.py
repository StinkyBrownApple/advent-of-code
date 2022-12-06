def main():
    line = open('input.txt').read()
    for i in range(len(line)):
        packet = line[i:i + 14]
        if len(packet) == len(set(packet)):
            return i + 14


if __name__ == '__main__':
    print(main())
