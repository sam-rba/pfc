package main

import "math"

func parseConstant(s string) (float64, error) {
	switch s {
	case "pi":
		return math.Pi, nil
	case "e":
		return math.E, nil
	}
	return 0, InvalidConstantErr{s}
}

type InvalidConstantErr struct {
	s string
}

func (e InvalidConstantErr) Error() string {
	return "invalid constant: " + e.s
}
