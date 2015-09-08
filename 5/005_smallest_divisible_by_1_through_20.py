"""Smallest Multiple - https://projecteuler.net/problem=5

2520 is the smallest number that can be divided by each of the numbers from
1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the
numbers from 1 to 20?
"""


def get_prime_product(n):
    """Return a dictionary with prime keys and exponent values

    The product of all the keys raised to their exponent is the number

    For example: 20 = 2 ** 2 * 5 -> {2: 2, 5:1}
    """
    primes_to_20 = (2, 3, 5, 7, 11, 13, 17, 19)
    if n in primes_to_20:
        return {n: 1}

    result = {k: 0 for k in primes_to_20}
    for p in primes_to_20:
        while n % p == 0:
            result[p] += 1
            n /= p

    result = {k: v for k, v in result.iteritems() if v > 0}
    return result


numbers = range(20, 2, -1)
result = {}

for n in numbers:
    prime_product = get_prime_product(n)
    for k, v in prime_product.iteritems():
        if k not in result:
            result[k] = v
        elif v > result[k]:
            result[k] = v

final_number = 1
for k, v in result.iteritems():
    final_number *= k ** v

assert 232792560 == final_number
