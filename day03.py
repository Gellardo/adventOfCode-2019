#!/usr/bin/env python3

import sys

def step(position, op):
    if op[0] == "U":
        return (position[0], position[1]+1)
    if op[0] == "D":
        return (position[0], position[1]-1)
    if op[0] == "L":
        return (position[0]-1, position[1])
    if op[0] == "R":
        return (position[0]+1, position[1])

def get_path(line):
    current = (0,0)
    path = set()
    for op in line:
        for i in range(int(op[1:])):
            current = step(current, op)
            path.add(current)
    return path

def get_intersections(inp):
    paths = [get_path(l) for l in inp]
    return paths[0].intersection(paths[1])

def part1(inp):
    intersections = get_intersections(inp)
    distances = list(map(lambda x: abs(x[0])+abs(x[1]), intersections))
    return min(distances)

assert(part1(["R75,D30,R83,U83,L12,D49,R71,U7,L72".split(","),
"U62,R66,U55,R34,D71,R55,D58,R83".split(",")]) == 159)
assert(part1(["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(","),
"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",")]) == 135)

lines = [ l.split(",") for l in sys.stdin ]
print("day 3 part 1:", part1(lines))

def get_steps_single(point, line):
    #print(point, line)
    current = (0,0)
    steps = 0
    for op in line:
        #print(op[0], end="")
        for i in range(int(op[1:])):
            #print(".", end="")
            current = step(current, op)
            steps += 1
            if current == point:
                break
        else:
            #print("")
            continue
        #print("x")
        break
    return steps

def get_steps(point, inp):
    s1 = get_steps_single(point, inp[0])
    s2 = get_steps_single(point, inp[1])
    print(s1,s2)
    return s1 + s2

def part2(inp):
    intersections = get_intersections(inp)
    #print(list(map(lambda inter: get_steps(inter, inp), intersections)))
    return min(map(lambda inter: get_steps(inter, inp), intersections))

assert(part2(["R8,U5,L5,D3".split(","), "U7,R6,D4,L4".split(",")]) == 30)
assert(part2(["R75,D30,R83,U83,L12,D49,R71,U7,L72".split(","),
"U62,R66,U55,R34,D71,R55,D58,R83".split(",")]) == 610)
assert(part2(["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(","),
"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",")]) == 410)

print("day 3 part 2:", part2(lines))
