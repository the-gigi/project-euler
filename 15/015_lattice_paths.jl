# https://projecteuler.net/problem=15
# answer: 137846528820

f20 = factorial(big(20))
f40 = factorial(big(40))
println(Int(f40 / f20 / f20))
