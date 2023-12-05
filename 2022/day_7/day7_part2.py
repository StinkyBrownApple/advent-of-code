class File:
    def __init__(self, name, parent_dir, file_size):
        self.name = name
        self.parent_dir = parent_dir
        self.file_size = file_size


class Dir:
    def __init__(self, name, parent_dir):
        self.contents = []
        self.dir_total_size = -1
        self.name = name
        self.parent_dir = parent_dir

    def set_contents(self, contents):
        self.contents = []
        for item in contents:
            if item.startswith('dir'):
                self.contents.append(Dir(item.split(" ")[1], self))
            else:
                file_info = item.split(' ')
                self.contents.append(File(file_info[1], self, int(file_info[0])))

    def find_dir(self, dir_name):
        for item in self.contents:
            if type(item) is Dir:
                if item.name == dir_name:
                    return item

    def get_dir_size(self):
        if self.dir_total_size > -1:
            return self.dir_total_size
        total = 0
        for item in self.contents:
            total += item.file_size if type(item) is File else item.get_dir_size()
        self.dir_total_size = total
        return total

    def populate_list(self, size_list, min_size):
        for item in self.contents:
            if type(item) is Dir:
                item.populate_list(size_list, min_size)
        if self.get_dir_size() >= min_size:
            size_list.append(self.get_dir_size())


def build_file_tree():
    root = Dir('/', None)
    working_dir = root
    lsing = False
    dir_contents = []
    for line in open('input.txt'):
        line = line.strip()
        if line.startswith('$'):
            if lsing:
                lsing = False
                working_dir.set_contents(dir_contents)
                dir_contents = []
            if line.startswith('$ cd '):
                cd_to = line[5:]
                if cd_to == '/':
                    working_dir = root
                elif cd_to == '..':
                    working_dir = working_dir.parent_dir
                else:
                    working_dir = working_dir.find_dir(cd_to)
            elif line == '$ ls':
                lsing = True
        elif lsing:
            dir_contents.append(line)
    if lsing:
        working_dir.set_contents(dir_contents)
    return root


def traverse_tree(file_tree):
    filesystem_size = 70000000
    required_space = 30000000
    root_size = file_tree.get_dir_size()
    free_space = filesystem_size - root_size
    space_to_free = required_space - free_space
    size_list = []
    file_tree.populate_list(size_list, space_to_free)
    size_list.sort()
    return size_list[0]


if __name__ == '__main__':
    print((traverse_tree(build_file_tree())))

