import math


def findPrimes(max):
    primes = []
    for dividend in range(1, max + 1):
        isPrime = True
        for dividor in range(2, dividend):
            if dividend / dividor == math.floor(dividend / dividor):
                isPrime = False
                break
        if isPrime == True:
            primes.append(dividend)

    return primes


primes = findPrimes(50)
print(primes)
