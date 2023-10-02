package main

type Stack []float64

func (s *Stack) push(v float64) {
	*s = append(*s, v)
}

func (s *Stack) pop() (float64, error) {
	if len(*s) > 0 {
		v := (*s)[len(*s)-1]
		*s = (*s)[:len(*s)-1]
		return v, nil
	}
	return 0, EmptyStackErr{}
}

type EmptyStackErr struct{}

func (e EmptyStackErr) Error() string {
	return "empty stack"
}
