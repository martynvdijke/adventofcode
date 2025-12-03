
import re

file_path = "example.txt"

# Regex to match L or R followed by one or more digits (e.g., "L68", "R123")
pattern = r'[LR]\d+'
start = 50
first_answer = []
second_answer = []
with open(file_path, "r") as file:
    file_list = file.read()
    split_lines = file_list.splitlines()
    for line in split_lines:
        match = re.search(pattern, line)
        direction = match.group()[0]
        number = int(match.group()[1:])
        click =- False
        if direction == 'L':
            if start - number < 0:
                start = 100 - (number - start)
                second_answer.append(1)
                click = True
            else:
                start -= number
        elif direction == 'R':
            if start + number > 100:
                start = number - (100 - start)
                second_answer.append(1)
                click = True
            else:
                start += number

        if click is False and (start == 100 or start == 0):
            second_answer.append(1)
        start = start % 100
        first_answer.append(start)
 
    print("First answer:")
    print(first_answer.count(0))
    print("Second answer:")
    print(sum(second_answer))

