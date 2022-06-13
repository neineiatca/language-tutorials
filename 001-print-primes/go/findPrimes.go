package main

import "fmt"

func findPrimes(max int) []int {
	var primes []int
	for dividend := 1; dividend <= max; dividend++ {
		isPrime := true
		for dividor := 2; dividor < dividend; dividor++ {
			intResult := dividend / dividor
			converted := float32(intResult)
			floatResult := float32(dividend) / float32(dividor)
			if converted == floatResult {
				isPrime = false
				break
			}
		}
		if isPrime {
			primes = append(primes, dividend)
		}
	}
	return primes
}

func main() {
	primes := findPrimes(100)
	fmt.Println(primes)
}
