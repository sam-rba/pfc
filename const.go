package main

import "math"

// parseConstant returns nil if s is not a valid constant.
func parseConstant(s string) *float64 {
	switch s {
	case "pi":
		// Assign to variable because can't take address of constant.
		var pi float64 = math.Pi
		return &pi
	}
	return nil
}
