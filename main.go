package main

import (
	"fmt"
	"os"

	"github.com/charmbracelet/bubbletea"
)

type Calculator struct {
	stack  Stack
	buffer string
}

type Stack []float64

func main() {
	if _, err := tea.NewProgram(new(UI)).Run(); err != nil {
		fmt.Fprintf(os.Stderr, "%v", err)
		os.Exit(1)
	}
}
