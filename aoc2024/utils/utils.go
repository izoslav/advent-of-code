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
	return string(data)
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

// functional

type mapFunc[E any] func(E) E

func Map[S ~[]E, E any](s S, f mapFunc[E]) S {
	result := make(S, len(s))
	for i := range s {
		result[i] = f(s[i])
	}
	return result
}

type filterFunc[E any] func(E) bool

func Filter[S ~[]E, E any](s S, f filterFunc[E]) S {
	result := S{}
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

type foldFunc[R any, E any] func(acc R, next E) R

func Fold[R any, E any](s []E, init R, f foldFunc[R, E]) R {
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
