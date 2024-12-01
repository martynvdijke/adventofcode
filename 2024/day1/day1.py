number1_list = []
number2_list = []
with open("input.txt", "r") as file:
    file_list = file.read()
    split_lines = file_list.splitlines()
    for line in split_lines:
        numbers = line.split()
        number1_list.append(int(numbers[0]))
        number2_list.append(int(numbers[1]))

number1_list.sort()
number2_list.sort()

## part A
sum = 0
for i in range(0, len(number1_list)):
    number1 = number1_list[i]
    number2 = number2_list[i]
    distance = abs(number2 - number1)
    sum += distance

print(sum)

## part B
similarity = 0
for i in range(0, len(number1_list)):
    number1 = number1_list[i]
    number2 = number2_list[i]
    if number1 in number2_list:
        times = number2_list.count(number1)
        score = number1*times
        similarity += score
        
print(similarity)