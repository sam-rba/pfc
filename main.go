package main

import (
	"fmt"
	"os"

	"github.com/charmbracelet/bubbletea"
)

// Types

type Calculator struct {
	stack  []float64
	buffer string
}

// Function Definitions

func main() {
	if _, err := tea.NewProgram(new(UI)).Run(); err != nil {
		fmt.Fprintf(os.Stderr, "%v", err)
		os.Exit(1)
	}
}
