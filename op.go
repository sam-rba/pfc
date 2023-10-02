package main

import (
	"fmt"
	"math"
)

// parseOperator returns a closure that performs the specified arithmetic operation,
// or OpError if op is not a valid operator.
func parseOperator(op byte) (func(lhs float64, rhs float64) float64, error) {
	switch op {
	case '+':
		return func(lhs, rhs float64) float64 { return lhs + rhs }, nil
	case '-':
		return func(lhs, rhs float64) float64 { return lhs - rhs }, nil
	case '*':
		return func(lhs, rhs float64) float64 { return lhs * rhs }, nil
	case '/':
		return func(lhs, rhs float64) float64 {
			if rhs != 0 {
				return lhs / rhs
			}
			return lhs
		}, nil
	case '%':
		return func(lhs, rhs float64) float64 {
			if rhs != 0 {
				return float64(int64(lhs) % int64(rhs))
			}
			return lhs
		}, nil
	case '^':
		return func(lhs, rhs float64) float64 { return math.Pow(lhs, rhs) }, nil
	}
	return nil, OperatorErr{op}
}

// OperatorErr records an invalid arithmetic operator.
type OperatorErr struct {
	c byte
}

func (e OperatorErr) Error() string {
	return fmt.Sprintf("invalid operator: %c", e.c)
}
