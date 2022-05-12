p = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
r = 8444461749428370424248824938781546531375899335154063827935233455917409239041

Cx.<x> = GF(p)[]
β = (x**3-1).roots()[0][0]
CX.<X> = GF(r)[]
λ = (X**3-1).roots()[1][0]
M = Matrix([[-ZZ(λ), 1], [r, 0]])
N = M.LLL()
N_inv = N**-1

k = 0x10A7D84E49ACCEBAAEC5ECB5C7EEC6021EF5356855CCE10EB79EFE317C42A623
beta = vector([k,0]) * N_inv
print('β1=',beta[0])
print('β2=',beta[1])
print('tβ1=',int(beta[0]))
print('tβ2=',int(beta[1]))
b = vector([int(beta[0]), int(beta[1])]) * N
print('b1=',b[0])
print('b2=',b[1])
k1 = k-b[0]
k2 = -b[1]

print('k1=',k1)
print('k2=',k2)