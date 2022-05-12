p = 6891450384315732539396789682275657542479668912536150109513790160209623422243491736087683183289411687640864567753786613451161759120554247759349511699125301598951605099378508850372543631423596795951899700429969112842764913119068299
r = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177

proof.arithmetic(False)
Cx.<x> = GF(p)[]
β = (x**3-1).roots()[0][0]

def endo(P):
    return P.parent()(P[0] * β, P[1])
CX.<X> = GF(r)[]
λ = (X**3-1).roots()[1][0]
M = Matrix([[-ZZ(λ), 1], [r, 0]])
N = M.LLL()
# we want det(N) == r
if det(N) == -r :
    N[1] *= -1

def print_scalar(x, name='x', field='Fr'):
    print("const {}: Self::ScalarField =".format(name))
    print("\tfield_new!({}, \"{}\");".format(field, x))


k = 0x0199E1755A3A4CDFA17454DBA7FA7AC40715E575B43A4ED1F47408DC2E2C97D5433A142AF15266349437B343298B4B94
print(hex(k))
for nn in (~N).coefficients():
    print(nn)
beta = vector([k,0]) * N**-1
print("β1 = ", int(beta[0]))
print("β2 = ", int(beta[1]))
b = vector([int(beta[0]), int(beta[1])]) * N
print("b1=", b[0])
print("b2=", b[1])
k1 = k-b[0]
k2 = -b[1]
print("k1=", hex(k1))
print("k2=", hex(k2))

E = EllipticCurve(GF(p), [0, -1])
g = E([0x01075B020EA190C8B277CE98A477BEAEE6A0CFB7551B27F0EE05C54B85F56FC779017FFAC15520AC11DBFCD294C2E746A17A54CE47729B905BD71FA0C9EA097103758F9A280CA27F6750DD0356133E82055928ACA6AF603F4088F3AF66E5B43D, 0x0058B84E0A6FC574E6FD637B45CC2A420F952589884C9EC61A7348D2A2E573A3265909F1AF7E0DBAC5B8FA1771B5B806CC685D31717A4C55BE3FB90B6FC2CDD49F9DF141B3053253B2B08119CAD0FB93AD1CB2BE0B20D2A1BAFC8F2DB4E95363])
