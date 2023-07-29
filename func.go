package main

import (
	"fmt"
	"math"
)

// parseFunction returns nil is fn is not a valid function.
func parseFunction(fn string) func(Calculator) {
	switch fn {
	case "sin", "cos", "tan":
		return trig(fn)
	case "asin", "acos", "atan":
		return invTrig(fn)
	case "deg":
		return deg
	case "rad":
		return rad
	}
	return nil
}

// trig returns a closure that performs the trig function specified by fn.
// Panics if fn is not one of "sin", "cos" or "tan".
func trig(fn string) func(Calculator) {
	return func(c Calculator) {
		if len(c.stack) <= 0 {
			return
		}
		v := &c.stack[len(c.stack)-1]
		// The math package expects arguments to trig functions to be in radians.
		if c.anglem == modeDeg {
			*v = radians(*v)
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
func invTrig(fn string) func(Calculator) {
	return func(c Calculator) {
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
		if c.anglem == modeDeg {
			*v = degrees(*v)
		}
	}
}

func deg(c Calculator) {
	if len(c.stack) > 0 {
		c.stack[len(c.stack)-1] = degrees(c.stack[len(c.stack)-1])
	}
}

func rad(c Calculator) {
	if len(c.stack) > 0 {
		c.stack[len(c.stack)-1] = radians(c.stack[len(c.stack)-1])
	}
}

// radians converts degrees to radians.
func radians(deg float64) float64 {
	return deg * math.Pi / 180.0
}

// degrees converts radians to degrees.
func degrees(rad float64) float64 {
	return rad * 180 / math.Pi
}
