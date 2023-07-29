package main

import (
	"fmt"
	"math"
)

// parseOp returns a closure that performs the specified arithmetic operation,
// or OpError if op is not a valid operator.
func parseOp(op byte) (func(lhs *float64, rhs float64), error) {
	switch op {
	case '+':
		return func(lhs *float64, rhs float64) { *lhs += rhs }, nil
	case '-':
		return func(lhs *float64, rhs float64) { *lhs -= rhs }, nil
	case '*':
		return func(lhs *float64, rhs float64) { *lhs *= rhs }, nil
	case '/':
		return func(lhs *float64, rhs float64) {
			if rhs != 0 {
				*lhs /= rhs
			}
		}, nil
	case '%':
		return func(lhs *float64, rhs float64) {
			if rhs != 0 {
				*lhs = float64(int64(*lhs) % int64(rhs))
			}
		}, nil
	case '^':
		return func(lhs *float64, rhs float64) { *lhs = math.Pow(*lhs, rhs) }, nil
	}
	return nil, OpError{op}
}

// OpError records an invalid arithmetic operator.
type OpError struct {
	c byte
}

func (e OpError) Error() string {
	return fmt.Sprintf("invalid operator: %c", e.c)
}
