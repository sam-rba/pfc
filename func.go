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
		return deg
	case "rad":
		return rad
	case "fac":
		return fac
	case "ch": // choose
		return combination
	case "log10":
		return log10
	case "log2":
		return log2
	case "ln":
		return ln
	}
	return nil
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

// Convert radians to degrees.
func deg(c *Calculator) {
	if n, err := c.stack.pop(); err == nil {
		c.stack.push(radToDeg(n))
	}
}

// Convert degrees to radians.
func rad(c *Calculator) {
	if n, err := c.stack.pop(); err == nil {
		c.stack.push(degToRad(n))
	}
}

// Factorial (!).
func fac(c *Calculator) {
	n, err := c.stack.pop()
	if err != nil {
		return
	}
	if !isUint(n) { // undefined on non-ints
		c.stack.push(n)
		return
	}
	c.stack.push(float64(factorial(uint(n))))
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

func degToRad(deg float64) (rad float64) {
	return deg * math.Pi / 180.0
}

func radToDeg(rad float64) (deg float64) {
	return rad * 180 / math.Pi
}

// factorial returns n! (n factorial).
func factorial(n uint) uint {
	if n == 0 { // 0! = 1
		return 1
	}
	// n! = n*(n-1)!
	for i := n - 1; i > 1; i-- {
		n *= i
	}
	return n
}

// decimal logarithm.
func log10(c *Calculator) {
	x, err := c.stack.pop()
	if err != nil {
		return
	}
	c.stack.push(math.Log10(x))
}

// binary logarithm.
func log2(c *Calculator) {
	x, err := c.stack.pop()
	if err != nil {
		return
	}
	c.stack.push(math.Log2(x))
}

// natural logarithm.
func ln(c *Calculator) {
	x, err := c.stack.pop()
	if err != nil {
		return
	}
	c.stack.push(math.Log(x))
}

func isUint(n float64) bool {
	return float64(uint(n)) == n
}
