package main

import "strconv"

type AngleMode bool

const (
	degrees AngleMode = false
	radians AngleMode = true
)

func (a AngleMode) String() string {
	if a == degrees {
		return "deg"
	}
	return "rad"
}

type Calculator struct {
	stack     Stack
	buf       string
	angleMode AngleMode
}

// swap swaps the values of the buffer and the bottom element of the stack. If
// the buffer is empty this simply pops from the stack. If the stack is empty,
// this simply pushes to the stack.
func (c *Calculator) swap() {
	stackVal, err := c.stack.pop()
	stackIsEmpty := err != nil

	if v, err := c.parseBuffer(); err == nil {
		c.stack.push(v)
	}

	if stackIsEmpty {
		c.buf = ""
	} else {
		c.buf = printNum(stackVal)
	}
}

// negate negates the number in the buffer, if any; or the bottom number on the
// stack, if any.
func (c *Calculator) negate() {
	if v, err := c.parseBuffer(); err == nil {
		c.buf = printNum(-v)
	} else if len(c.buf) == 0 && len(c.stack) > 0 {
		c.stack[len(c.stack)-1] = -c.stack[len(c.stack)-1]
	}
}

// performOp performs the specified arithmetic operation.
func (c *Calculator) performOperation(operator byte) {
	fn, err := parseOperator(operator)
	if err != nil {
		return
	}
	lhs, rhs, err := c.operands()
	if err != nil {
		return
	}
	c.stack.push(fn(lhs, rhs))
	c.buf = ""
}

// operands returns the left and right operands, or error if there are not enough.
func (c *Calculator) operands() (lhs, rhs float64, err error) {
	if rhs, err = c.parseBuffer(); err == nil {
		lhs, err = c.stack.pop()
		return
	} else if rhs, err = c.stack.pop(); err == nil {
		lhs, err = c.stack.pop()
		if err != nil { // missing lhs
			c.stack.push(rhs)
		}
		return
	}
	return 0, 0, OperandErr{}
}

// parseBuffer returns the numerical value of the contents of the buffer.
func (c Calculator) parseBuffer() (float64, error) {
	if con, err := parseConstant(c.buf); err == nil {
		return con, nil
	} else if fl, err := strconv.ParseFloat(c.buf, 64); err == nil {
		return fl, nil
	}
	return 0, InvalidBufferContentErr{c.buf}
}

type InvalidBufferContentErr struct {
	buf string
}

func (e InvalidBufferContentErr) Error() string {
	return "invalid buffer contents: " + e.buf
}

type OperandErr struct{}

func (e OperandErr) Error() string {
	return "not enough operands"
}
