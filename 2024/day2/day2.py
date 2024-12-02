import numpy as np

def read_input():
    output = []
    with open("input.txt", "r") as file:
        file_list = file.read()
        split_lines = file_list.splitlines()
        for line in split_lines:
            level = []
            numbers = line.split()
            for number in numbers:
                level.append(int(number))
            output.append(level)
    return output

def part1():
    test = read_input()

    safe = 0
    max = 0
    for level in test:
        np_array = np.array(level)
        grad = np.gradient(np_array)
        test = ((grad > 0).all() and (grad > -2.).all())
        print("test")
        # if np.all(grad, where= ):
        #     print("test")



def part2():
    pass

part1()
part2()
