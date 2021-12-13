#!/usr/bin/env python3
example_data = ["[({(<(())[]>[[{[]{<()<>>",
                "[(()[<>])]({[<{<<[]>>(",
                "{([(<{}[<>[]}>{[]{[(<()>",
                "(((({<>}<{<{<>}{[]{[]{}",
                "[[<[([]))<([[{}[[()]]]",
                "[{[{({}]{}}([{[{{{}}([]",
                "{<[[]]>}<{[{[{[]{()[[[]",
                "[<(<(<(<{}))><([]([]()",
                "<{([([[(<>()){}]>(<<{{",
                "<{([{{}}[<[[[<>{}]]]>[]]"]

from functools import reduce

with open("input.txt") as f:
    input_data = f.read().splitlines()

def parse(line):
    opening_chars = {"(": ")", "[": "]", "{": "}", "<": ">"}
    tally = []
    for c in line:
        if c in opening_chars:
            tally.append(opening_chars[c])
        elif c != tally.pop():
            return {"last_char": c, "tally": []}
    return {"tally": tally}

def syntax_score(m):
    scoring = {")": 3, "]": 57, "}": 1197, ">": 25137, None: 0}
    return scoring[m.get("last_char")]

def completion_score(m):
    scoring = {")": 1, "]": 2, "}": 3, ">": 4}
    return reduce(lambda x, y: x * 5 + scoring[y], reversed(m["tally"]), 0)

print(f'Puzzle1: {sum(syntax_score(parse(l)) for l in input_data)}')

scores = sorted(filter(bool, (completion_score(parse(l)) for l in input_data)))
print(f'Puzzle2: {scores[len(scores) // 2]}')
