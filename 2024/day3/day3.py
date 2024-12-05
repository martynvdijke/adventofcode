def read_input():
    output = []
    with open("input.txt", "r") as file:
        file_list = file.read()
        split_lines = file_list.splitlines()
        for line in split_lines:
            output.append(int(line))
    return output


def part1():
    #mul\(\d{1,},\d{1,}\)
    #\d{1,}
    output = read_input()
    multis = []
    for i in range(0, (len(output) - 1), 2):
        mul = output[i] * output[i + 1]
        # print(mul)
        multis.append(mul)
    
    sum = 0
    for multi in multis:
        sum += multi

    print(sum)
part1()
