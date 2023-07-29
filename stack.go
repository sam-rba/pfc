package main

type Stack []float64

func (s *Stack) push(v float64) {
	*s = append(*s, v)
}

func (s *Stack) pop() *float64 {
	if len(*s) > 0 {
		v := (*s)[len(*s)-1]
		*s = (*s)[:len(*s)-1]
		return &v
	}
	return nil
}
