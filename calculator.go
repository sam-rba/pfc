package main

import "strconv"

type Stack []float64

type Calculator struct {
	stack  Stack
	buffer string
}

// add performs addition when the user inputs the '+' operator.
func (c *Calculator) add() {
	if len(c.stack) < 1 {
		return
	}
	if con := parseConstant(c.buffer); con != nil {
		c.stack[len(c.stack)-1] += *con
	} else if f, err := strconv.ParseFloat(c.buffer, 64); err == nil {
		c.stack[len(c.stack)-1] += f
	} else if len(c.stack) > 1 {
		c.stack[len(c.stack)-2] += c.stack[len(c.stack)-1]
		c.stack = c.stack[:len(c.stack)-1]
	}
	c.buffer = ""
}
