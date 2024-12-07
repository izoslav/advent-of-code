package utils

import (
	"math"
	"os"
	"strconv"
	"strings"
)

// file operations

func ReadFile(filepath string) string {
	data, _ := os.ReadFile(filepath)
	return strings.TrimSpace(string(data))
}

func ReadLines(filepath string) []string {
	data := ReadFile(filepath)
	return strings.Split(data, "\n")
}

// conversions

func Atoi(input string) int {
	value, _ := strconv.Atoi(input)
	return value
}

func Atoi64(input string) int64 {
	value, _ := strconv.ParseInt(input, 10, 64)
	return value
}

// functional

type mapFunc[E any, R any] func(E) R

func Map[E, R any](s []E, f mapFunc[E, R]) []R {
	result := make([]R, len(s))
	for i := range s {
		result[i] = f(s[i])
	}
	return result
}

type filterFunc[E any] func(E) bool

func Filter[E any](s []E, f filterFunc[E]) []E {
	result := []E{}
	for _, e := range s {
		if f(e) {
			result = append(result, e)
		}
	}
	return result
}

type reduceFunc[E any] func(acc E, next E) E

func Reduce[E any](s []E, init E, f reduceFunc[E]) E {
	acc := init
	for _, v := range s {
		acc = f(acc, v)
	}
	return acc
}

type foldFunc[E, R any] func(acc R, next E) R

func Fold[E, R any](s []E, init R, f foldFunc[E, R]) R {
	acc := init
	for _, v := range s {
		acc = f(acc, v)
	}
	return acc
}

type Pair[E any] struct {
	Left  E
	Right E
}

func Zip[E any](left []E, right []E) []Pair[E] {
	result := make([]Pair[E], len(left))
	for i := range left {
		result[i] = Pair[E]{
			Left:  left[i],
			Right: right[i],
		}
	}
	return result
}

// math

func AbsInt(value int) int {
	return int(math.Abs(float64(value)))
}
