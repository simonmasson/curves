from sage.all import *
from sage.rings.finite_rings.finite_field_constructor import FiniteField

'''
Generating a Twisted Edwards curve defined over p_pallas for tests...
'''
p = 28948022309329048855892746252171976963363056481941560715954676764349967630337
Fp = FiniteField(p)
Fpz = Fp['z']
z = Fpz.gen()

# a,b = Fp.random_element(), Fp.random_element()
a,b = Fp(15472856019507685383953719555290536676206397041956899727279880177693842823075), Fp(24419954351426524683892915143322266162905124037100658665599160753587064349619)
E = EllipticCurve([a,b])
OrderE = E.order()
alpha = 0
while alpha == 0 or (3*alpha**2 + a).is_square == False:
    while OrderE%4 != 0 or OrderE.factor()[-1][0] < (1<<100):
        a,b = Fp.random_element(), Fp.random_element()
        E = EllipticCurve([a,b])
        OrderE = E.order()
    alpha = (z**3 + a*z+b).roots()[0][0]

# OrderE % 64986179019709162267305887466992519 = 0
r = 64986179019709162267305887466992519
G_ws = E(10587733790699609394260227672753853646051912153269630680244231899621041722577,22041641853171658531264670636745512177619060565997377443928240445297773222283)

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
