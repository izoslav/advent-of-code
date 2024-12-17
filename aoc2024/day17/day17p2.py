from z3 import *

# a = 66752888
# b = 0
# c = 0
# Program: 2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0
# 2 4 - bst 4 - b = a % 8
# 1 7 - bxl 7 - b = b ^ 7
# 7 5 - cdv 5 - c = a / 2**b
# 1 7 - bxl 7 - b = b ^ 7
# 0 3 - adv 3 - a = a / 8
# 4 1 - bxc 1 - b = b ^ c
# 5 5 - out 5 - out b % 8
# 3 0 - jnz 0 - jump if a != 0

opt = Optimize()
s = BitVec('s', 64)

a, b, c = s, 0, 0

for x in [2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0]:
    b = a % 8
    b = b ^ 7
    c = a / (1 << b)
    b = b ^ 7
    a = a / (1 << 3)
    b = b ^ c
    opt.add((b % 8) == x)
opt.add(a == 0)
opt.minimize(s)

assert str(opt.check()) == 'sat'
print(opt.model().eval(s))
