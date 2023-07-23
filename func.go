package main

import "math"

// parseFunction returns nil is s is not a valid function.
func parseFunction(s string) func(Stack) {
	switch s {
	case "sin":
		return sin
	}
	return nil
}

func sin(stack Stack) {
	if len(stack) > 0 {
		stack[len(stack)-1] = math.Sin(stack[len(stack)-1])
	}
}
