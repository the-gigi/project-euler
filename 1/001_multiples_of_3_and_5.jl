# https://projecteuler.net/problem=1
#If we list all the natural numbers below 10 that are multiples of 3 or 5,
# we get 3, 5, 6 and 9. The sum of these multiples is 23.
#
#Find the sum of all the multiples of 3 or 5 below 1000.

elements = Set()

function add_multiples(number, elements)
    i = number
    while i < 1000
        push!(elements, i)
        i += number
    end
end

add_multiples(3, elements)
add_multiples(5, elements)

println(sum(elements))
