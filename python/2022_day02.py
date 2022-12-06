#!/usr/bin/env python3
from aocd import get_data, submit
puzzle = get_data(day=2, year=2022)

# A/X Rock
# B/Y Paper
# C/Z Scissor
shape_points = {'X': 1, 'Y': 2, 'Z': 3}
win_points = {'X': {'A': 3, 'B': 0, 'C': 6},
              'Y': {'A': 6, 'B': 3, 'C': 0},
              'Z': {'A': 0, 'B': 6, 'C': 3}}

acc = 0
for round in puzzle.split('\n'):
    opponent, myself = round.split(' ')
    points = shape_points[myself] + win_points[myself][opponent]
    acc += points

# submit(acc, part="a", day=2, year=2022)

# X lose
# Y draw
# z win
conforming_shape = {'A': {'X': 'Z', 'Y': 'X', 'Z': 'Y'},
                    'B': {'X': 'X', 'Y': 'Y', 'Z': 'Z'},
                    'C': {'X': 'Y', 'Y': 'Z', 'Z': 'X'}}
acc = 0
for round in puzzle.split('\n'):
    opponent, strategy = round.split(' ')
    myself = conforming_shape[opponent][strategy]
    points = shape_points[myself] + win_points[myself][opponent]
    acc += points

submit(acc, part='b', day=2, year=2022)
