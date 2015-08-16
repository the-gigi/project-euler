"""Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
"""

from collections import OrderedDict


def find_next_prime(primes):
    last = primes[-1]
    candidates = OrderedDict((k, 1) for k in range(last + 1, last * 2 + 1))

    for p in primes:
        for i in xrange(p * 2, last * 2 + 1, p):
            candidates[i] = 0

    for k, v in candidates.iteritems():
        if v == 1:
            return k


def find_largest_prime_factor(number):
    largest = 1
    primes = [2]

    while True:
        last = primes[-1]
        if number % last == 0:
            largest = last
            #print largest, primes

            number /= last
            # Keep dividing
            while number % last == 0:
                number /= last

            if number == 1:
                return largest

        primes.append(find_next_prime(primes))

def main():
    number = 600851475143
    largest = find_largest_prime_factor(number)
    print largest

if __name__ == '__main__':
    main()
