package main

import (
	"fmt"
	"math"
	"strconv"
)

type Stack []float64

type Calculator struct {
	stack Stack
	buf   string
}

// performOp performs the specified arithmetic operation and returns nil or
// OpError if op is not a valid operator.
func (c *Calculator) performOp(op byte) error {
	if len(c.stack) < 1 {
		return nil
	}

	fn, err := parseOp(op)
	if err != nil {
		return err
	}

	if con := parseConstant(c.buf); con != nil {
		fn(&c.stack[len(c.stack)-1], *con)
	} else if fl, err := strconv.ParseFloat(c.buf, 64); err == nil {
		fn(&c.stack[len(c.stack)-1], fl)
	} else if len(c.stack) > 1 {
		fn(&c.stack[len(c.stack)-2], c.stack[len(c.stack)-1])
		c.stack = c.stack[:len(c.stack)-1]
	}
	c.buf = ""
	return nil
}

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
