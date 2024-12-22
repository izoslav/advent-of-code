package main

import (
	"fmt"
	"math"

	"github.com/izoslav/aoc2024/utils"
)

func nextSecret(secret int) int {
	secret = prune(mix(secret, secret<<6))

	result := int(math.Round(float64(secret)) / 32)
	secret = prune(mix(secret, result))

	secret = prune(mix(secret, secret<<11))
	return secret
}

func mix(secret int, mixer int) int {
	return mixer ^ secret
}

func prune(secret int) int {
	return secret % 16777216
}

func solve(ids []int, rounds int) (secretsSum int, maxPrice int) {
	maxPrices := map[string]int{}
	for _, id := range ids {
		prices := []int{id % 10}

		secret := id
		for i := 0; i < rounds; i++ {
			secret = nextSecret(secret)
			prices = append(prices, secret%10)
		}

		changePrice := map[string]int{}
		changes := make([]int, len(prices)-1)
		for i := range changes {
			changes[i] = prices[i+1] - prices[i]
		}

		for i := 0; i < len(changes)-4; i++ {
			key := fmt.Sprintf("%d %d %d %d", changes[i], changes[i+1], changes[i+2], changes[i+3])
			if _, ok := changePrice[key]; !ok {
				changePrice[key] = max(changePrice[key], prices[i+4])
			}
		}

		for k, v := range changePrice {
			maxPrices[k] += v
		}

		secretsSum += secret
	}

	for _, price := range maxPrices {
		maxPrice = max(maxPrice, price)
	}

	return
}

func main() {
	// inputs := utils.ReadLines("day22/test-p1.txt")
	// inputs := utils.ReadLines("day22/test-p2.txt")
	inputs := utils.ReadLines("day22/input.txt")

	ids := make([]int, len(inputs))
	for i, id := range inputs {
		ids[i] = utils.Atoi(id)
	}

	secretSum, bestTrade := solve(ids, 2000)

	fmt.Println("=== day 22 ===")
	fmt.Println("part 1:", secretSum)
	fmt.Println("part 2:", bestTrade)
	fmt.Println("==============")
}
