def findPrimes(max)
    primes = []
    for dividend in 1...max
        isPrime = true
        for dividor in 2...dividend
            if dividend / dividor == dividend.to_f / dividor.to_f
                isPrime = false
                break
            end
        end
        if isPrime
            primes.append(dividend)
        end
    end
    return primes
end

primes = findPrimes(100)
puts primes
