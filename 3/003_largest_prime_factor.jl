next_prime = # https://projecteuler.net/problem=3
# The prime factors of 13195 are 5, 7, 13 and 29.
# What is the largest prime factor of the number 600851475143 ?

function find_next_prime(start, stop, primes)
    candidates = []
    for p in primes
        candidates = ones(Int, stop - start)
        println(candidates)
        i = (start + p) % p
        while i < size(candidates, 1)
            println(i)
            candidates[i+1] = 0
            i += p
        end
    end
    println("-------")
    println(candidates)

    for i in 1:(stop - start + 1)
        if candidates[i] == 1
            return start + i - 1
        end
    end
end

primes = [2, 3, 5, 7]
next_prime = find_next_prime(10, 15, primes)
println(next_prime)
