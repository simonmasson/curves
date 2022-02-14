from sage.all import *
from sage.rings.finite_rings.finite_field_constructor import FiniteField

'''
Generating a Twisted Edwards curve defined over p_pallas for tests...
'''
p = 28948022309329048855892746252171976963363056481941647379679742748393362948097
Fp = FiniteField(p)
Fpz = Fp['z']
z = Fpz.gen()

# a,b = Fp.random_element(), Fp.random_element()
a= Fp(12033307035803878849683165116917446042199434082332695997656512932932991033641)
b= Fp(21529462793858023443483451437869885304357989953298403038493222394236648576789)
E = EllipticCurve([a,b])
OrderE = E.order()
alpha = 0
while alpha == 0 or (3*alpha**2 + a).is_square == False:
    while OrderE%4 != 0 or OrderE.factor()[-1][0] < (1<<100):
        a,b = Fp.random_element(), Fp.random_element()
        E = EllipticCurve([a,b])
        OrderE = E.order()
    alpha = (z**3 + a*z+b).roots()[0][0]

# OrderE % r = 0 with
r = 8166037775958011510003757022780509915545197219227314002537546727

G_ws = E(4356316195798653406553170569412604812177689345415962621018877128289734404066,4833359151334879449080234494095206083548078745547043403722701446818691755949)
# Montgomery model
s = 1/(3*alpha**2 + a).sqrt()
A_m = 3*alpha*s
B_m = s
G_m = [s*(G_ws[0] - alpha), s*G_ws[1]]
assert B_m * G_m[1]**2 == G_m[0]**3 + A_m * G_m[0]**2 + G_m[0]

# Twisted Edwards model
A_te = (A_m+2)/B_m
D_te = (A_m-2)/B_m
G_te = [G_m[0]/G_m[1], (G_m[0]-1)/(G_m[0]+1)]
assert A_te * G_te[0]**2 + G_te[1]**2 == 1 + D_te * G_te[0]**2 * G_te[1]**2
print('a_te = ', A_te)
print('d_te = ', D_te)
print('x=', G_te[0])
print('y=', G_te[1])

print('A_m = ', A_m)
print('B_m = ', B_m)
print('A_w = ', a)
print('B_w = ', b)
print('xw=', G_ws[0])
print('yw=', G_ws[1])
