package main

import "math"

const (
	// Vacuum permettivity [C²/(N·m²)]
	e0 = 8.8541878188e-12
)

func parseConstant(s string) (float64, error) {
	switch s {
	case "pi":
		return math.Pi, nil
	case "e":
		return math.E, nil
	case "e0":
		return e0, nil
	}
	return 0, InvalidConstantErr{s}
}

type InvalidConstantErr struct {
	s string
}

func (e InvalidConstantErr) Error() string {
	return "invalid constant: " + e.s
}
