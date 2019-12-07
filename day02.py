#!/usr/bin/env python3

raw = [l.strip() for l in open('inputs/day02').readlines()]
program_in = list(map(lambda x: [ int(y) for y in x.split(",")], raw))[0]

print([ x for x in program_in])

def part1(program):
    ops = {
        1: lambda x,y: program[x] + program[y],
        2: lambda x,y: program[x] * program[y]
    }
    for i in range(0, len(program), 4):
        op = program[i]
        if op == 99:
            break
        a, b, c = program[i+1:i+4]
        program[c] = ops[op](a,b)
    return program[0]

assert(part1([1,0,0,0,99]) == 2)
assert(part1([2,4,0,0,99]) == 99*2)
assert(part1([1,0,0,2,2,2,8,0,99]) == 99*2)
assert(part1([1,1,1,4,99,5,6,0,99]) == 30)

def part1_1(memory, noun, verb):
    program_in = [ x for x in memory ]

    program_in[1] = noun
    program_in[2] = verb
    return part1(program_in)

print("start")
part1_res = part1_1(program_in, 12,2)
print(part1_res == 4023471)
print("day 2 part 1:", part1_res)

def part2(memory, expected):
    for i in range(100):
        for j in range(100):
            if part1_1(memory, i, j) == expected:
                print(i, j)
                return 100*i+j

print("day 2 part 2", part2(program_in, 19690720))
