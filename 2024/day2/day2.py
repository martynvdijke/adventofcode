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
    for level in test:
        np_array = np.array(level)
        diff = np.ediff1d(np_array)
        decrease = (diff < 0).sum()
        increase = (diff > 0).sum()
        max_decrease = np.min(diff)
        max_increase = np.max(diff)
        if decrease == (len(level) - 1) and max_decrease >= -3:
            safe += 1
            continue
        if increase == (len(level) - 1) and max_increase <= 3:
            safe += 1
            continue

    print(safe)


def part2():
    test = read_input()

    safe = 0
    for level in test:
        np_array = np.array(level)
        diff = np.ediff1d(np_array)
        
        decrease = (diff < 0).sum()
        increase = (diff > 0).sum()
        max_decrease = np.min(diff)
        max_increase = np.max(diff)
        
        if decrease == (len(level) - 1) and max_decrease >= -3:
            safe += 1
            continue
        if increase == (len(level) - 1) and max_increase <= 3:
            safe += 1
            continue
        
        if decrease == (len(level) - 2):
            safe += decrease_func(level)
            continue

        if increase == (len(level) - 2):
            safe += increase_func(level)
            continue

    print(safe)


def increase_func(level):
    diff = np.ediff1d(np.array(level))
    for i in range(0, len(level)):
        temp2 = level.copy()
        temp2.pop(i)
        np_array = np.array(temp2)
        diff = np.ediff1d(np_array)
        max_increase = np.max(diff)
        increase = (diff > 0).sum()
        if max_increase <= 3 and increase == (len(level) - 2):
            return 1
    return 0


def decrease_func(level):
    diff = np.ediff1d(np.array(level))
    for i in range(0, len(level)):
        temp2 = level.copy()
        temp2.pop(i)
        np_array = np.array(temp2)
        diff = np.ediff1d(np_array)
        max_decrease = np.min(diff)
        decrease = (diff < 0).sum()
        if max_decrease >= -3 and decrease == (len(level) - 2):
            return 1
    return 0

part1()
part2()
