package main

import (
	"fmt"

	"github.com/charmbracelet/bubbletea"
)

// Types

type UI struct {
	calc        Calculator
	windowWidth int // Width of the window measured in characters.
}

// Interface Implementations

func (ui UI) Init() tea.Cmd {
	return nil
}

func (ui UI) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		ui.windowWidth = msg.Width
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return ui, tea.Quit
		}
	}
	return ui, nil
}

func (ui UI) View() string {
	var s string
	for _, f := range ui.calc.stack {
		s += fmt.Sprintf("%f\n", f)
	}
	horizBar := make([]byte, ui.windowWidth)
	for i := range horizBar {
		horizBar[i] = '-'
	}
	s += string(horizBar) + "\n"
	s += fmt.Sprintf("| %*s |\n", ui.windowWidth-4, ui.calc.buffer)
	s += string(horizBar)
	return s
}
