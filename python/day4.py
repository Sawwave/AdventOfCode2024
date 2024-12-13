import re

total_matches = 0
input_lines = []

def make_diag_word(input_lines, row, col):
    if (row+3) >= len(input_lines) or (col+3) >= len(input_lines[row]):
        return ''
    return input_lines[row][col] + input_lines[row+1][col+1] + input_lines[row+2][col+2] + input_lines[row+3][col+3]

def make_anti_diag_word(input_lines, row, col):
    if (row-3) < 0 or (col-3) < 0:
        return ''
    return input_lines[row][col] + input_lines[row-1][col-1] + input_lines[row-2][col-2] + input_lines[row-3][col-3]
def make_vertical_word(input_lines, row, col):
    if (row+3) >= len(input_lines):
        return ''
    return input_lines[row][col] + input_lines[row+1][col] + input_lines[row+2][col] + input_lines[row+3][col]

with open('inputs/day4.txt') as input_file:
    input_lines = input_file.readlines()

line_len = len(input_lines[0])
for line_idx in range(len(input_lines)):
    total_matches += len(re.findall('XMAS', input_lines[line_idx]))
    total_matches += len(re.findall('SAMX', input_lines[line_idx]))

for col_idx in range(line_len):
    column = ''.join(list(map(lambda line: line[col_idx], input_lines)))
    total_matches += len(re.findall('XMAS', column))
    total_matches += len(re.findall('SAMX', column))

for row_idx in range(len(input_lines)):
    for col_idx in range(line_len):
        diag_word = make_diag_word(input_lines, row_idx, col_idx)
        anti_diag_word = make_anti_diag_word(input_lines, row_idx, col_idx)
        if diag_word == "XMAS" or diag_word == "SAMX":
            total_matches += 1
        if anti_diag_word == "XMAS" or anti_diag_word == "SAMX":
            total_matches += 1
        total_matches += len(re.findall(diag_word, input_lines[row_idx]))
        total_matches += len(re.findall(anti_diag_word, input_lines[row_idx]))

print(total_matches)
