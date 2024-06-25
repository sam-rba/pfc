package main

import (
	"fmt"
	"math"
)

// parseFunction returns nil is fn is not a valid function.
func parseFunction(fn string) func(*Calculator) {
	switch fn {
	case "sin", "cos", "tan":
		return trig(fn)
	case "asin", "acos", "atan":
		return invTrig(fn)
	case "deg":
		return apply(radToDeg)
	case "rad":
		return apply(degToRad)
	case "fac":
		return apply(func(x float64) float64 {
			if !isUint(x) {
				return x
			}
			return float64(factorial(uint(x)))
		})
	case "ch": // choose
		return combination
	case "log10":
		return apply(math.Log10)
	case "log2":
		return apply(math.Log2)
	case "ln":
		return apply(math.Log)
	}
	return nil
}

// apply returns a function that applies fn to the bottom stack element of the
// calculator.
func apply(fn func(float64) float64) func(*Calculator) {
	return func(c *Calculator) {
		x, err := c.stack.pop()
		if err != nil {
			return
		}
		c.stack.push(fn(x))
	}
}

// trig returns a closure that performs the trig function specified by fn.
// Panics if fn is not one of "sin", "cos" or "tan".
func trig(fn string) func(*Calculator) {
	return func(c *Calculator) {
		if len(c.stack) <= 0 {
			return
		}
		v := &c.stack[len(c.stack)-1]
		// The math package expects arguments to trig functions to be in radians.
		if c.angleMode == degrees {
			*v = degToRad(*v)
		}
		switch fn {
		case "sin":
			*v = math.Sin(*v)
		case "cos":
			*v = math.Cos(*v)
		case "tan":
			*v = math.Tan(*v)
		default:
			panic(fmt.Sprintf("invalid trig function: '%s'", fn))
		}
	}
}

// invTrig returns a closure that performs the inverse trig function specified
// by fn. Panics if fn is not one of "asin", "acos" or "atan".
func invTrig(fn string) func(*Calculator) {
	return func(c *Calculator) {
		if len(c.stack) <= 0 {
			return
		}
		v := &c.stack[len(c.stack)-1]
		switch fn {
		case "asin":
			*v = math.Asin(*v)
		case "acos":
			*v = math.Acos(*v)
		case "atan":
			*v = math.Atan(*v)
		default:
			panic(fmt.Sprintf("invalid inverse trig function: '%s'", fn))
		}
		if c.angleMode == degrees {
			*v = radToDeg(*v)
		}
	}
}

func degToRad(deg float64) (rad float64) {
	return deg * math.Pi / 180.0
}

func radToDeg(rad float64) (deg float64) {
	return rad * 180 / math.Pi
}

// factorial returns n! (n factorial).
func factorial(n uint) uint {
	if n == 0 {
		return 1
	}
	// n! = n*(n-1)!
	for i := n - 1; i > 1; i-- {
		n *= i
	}
	return n
}

// Combination function. "n choose k" with integers n and k such that n >= k >= 0.
func combination(c *Calculator) {
	k, err := c.stack.pop()
	if err != nil {
		return
	}
	if !isUint(k) { // undefined on non-ints
		c.stack.push(k)
		return
	}

	n, err := c.stack.pop()
	if err != nil {
		c.stack.push(k)
		return
	}
	if !isUint(n) { // undefined on non-ints
		c.stack.push(n)
		return
	}

	if k > n || n < 0 || k < 0 {
		c.stack.push(n)
		c.stack.push(k)
	} else {
		n, k := uint(n), uint(k)
		c.stack.push(float64(factorial(n) / (factorial(k) * factorial(n-k))))
	}
}

func isUint(n float64) bool {
	return float64(uint(n)) == n
}
