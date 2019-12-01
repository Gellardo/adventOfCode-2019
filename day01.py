#!/usr/bin/env python3

import sys

lines = [ l for l in sys.stdin ]

def fuel_for_mass(inp):
    s = 0
    for l in inp:
        mass = int(l)
        s += mass // 3 - 2
    return s

assert(fuel_for_mass([12, 100756]) == 33585)

print("day 01 part 1:", fuel_for_mass(lines))

def include_fuel_mass(inp):
    s = 0
    for l in inp:
        mass = int(l)
        fuel = mass // 3 - 2
        s += fuel
        while True:
            fuel = fuel // 3 - 2
            if fuel > 0:
                s += fuel
            else:
                break

    return s

assert(include_fuel_mass([1969]) == 966)
assert(include_fuel_mass([100756]) == 50346)


print("day 01 part 2:", include_fuel_mass(lines))
