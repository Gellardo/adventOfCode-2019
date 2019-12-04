#!/usr/bin/env python3

from collections import defaultdict

def check_number(digits):
    return (len(digits) == 6 and
            any([ digits[i-1] == digits[i] for i in range(1, len(digits)) ]) and
            all([ digits[i-1] <=  digits[i] for i in range(1, len(digits)) ]))

def iterate(num_digits, digits, condition):
    start = 0 if len(digits) == 0 else digits[-1]
    if num_digits <= 0:
        if condition(digits):
            #print(digits)
            return 1
        return 0
    return sum(( iterate(num_digits-1, digits + [i], condition) for i in range(start, 10)))

def digits2num(digits):
    return sum([ digits[-i] * 10**(i-1) for i in range(1,len(digits)+1) ])

print(digits2num([1,2,3,4]))
print(iterate(2,[], lambda x: False))
print("day 4 part 1: ", iterate(6, [], lambda x: not (digits2num(x) < 172851 or digits2num(x) > 675869) and check_number(x)))

def check_extended(digits):
    digit_occ = defaultdict(lambda: 0)
    for d in digits: digit_occ[d] += 1
    return 2 in digit_occ.values()
assert(check_extended([1,1,1]) == False)
assert(check_extended([1,1,1,2,2,3]) == True)

print("day 4 part 2: ", iterate(6, [], lambda x: not (digits2num(x) < 172851 or digits2num(x) > 675869) and check_number(x) and
    check_extended(x)))

