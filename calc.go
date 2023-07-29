package main

import (
	"strconv"
	"strings"
)

type AngleMode int

const (
	modeDeg = iota
	modeRad
)

type Calculator struct {
	stack  Stack
	buf    string
	anglem AngleMode
}

// swap swaps the values of the buffer and the bottom element of the stack. If
// the buffer is empty this simply pops from the stack. If the stack is empty,
// this simply pushes to the stack.
func (c *Calculator) swap() {
	st := c.stack.pop()
	if con := parseConstant(c.buf); con != nil {
		c.stack.push(*con)
	} else if f, err := strconv.ParseFloat(c.buf, 64); err == nil {
		c.stack.push(f)
	}
	if st != nil {
		c.buf = strings.TrimSpace(printNum(*st))
	} else {
		c.buf = ""
	}
}

// negate negates the number in the buffer, if any; or the bottom number on the
// stack, if any.
func (c *Calculator) negate() {
	if con := parseConstant(c.buf); con != nil {
		c.buf = strings.TrimSpace(printNum(-*con))
	} else if f, err := strconv.ParseFloat(c.buf, 64); err == nil {
		c.buf = strings.TrimSpace(printNum(-f))
	} else if len(c.buf) == 0 && len(c.stack) > 0 {
		c.stack[len(c.stack)-1] = -c.stack[len(c.stack)-1]
	}
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
		fn(&c.stack[len(c.stack)-2], *c.stack.pop())
	}
	c.buf = ""
	return nil
}
