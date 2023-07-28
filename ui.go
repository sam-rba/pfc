package main

import (
	"fmt"
	"strconv"

	"github.com/charmbracelet/bubbletea"
)

// UTF-8 box drawing characters.
const (
	boxHorizontal  = '─'
	boxVertical    = '│'
	boxTopLeft     = '┌'
	boxTopRight    = '┐'
	boxBottomLeft  = '└'
	boxBottomRight = '┘'
)

// sigDigs is the number of significant digits when printing a number.
const sigDigs = 64

type UI struct {
	calc        Calculator
	windowWidth int // Width of the window measured in characters.
}

func (ui UI) Init() tea.Cmd {
	return nil
}

func (ui UI) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		ui.windowWidth = msg.Width
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "Q":
			return ui, tea.Quit
		case "+", "-", "*", "/", "%", "^":
			if err := ui.calc.performOp(msg.String()[0]); err != nil {
				panic(err)
			}
		case "backspace":
			if len(ui.calc.buf) > 0 {
				ui.calc.buf = ui.calc.buf[:len(ui.calc.buf)-1]
			}
		case "enter":
			if fn := parseFunction(ui.calc.buf); fn != nil {
				fn(ui.calc.stack)
			} else if con := parseConstant(ui.calc.buf); con != nil {
				ui.calc.stack = append(ui.calc.stack, *con)
			} else if f, err := strconv.ParseFloat(ui.calc.buf, 64); err == nil {
				ui.calc.stack = append(ui.calc.stack, f)
			}
			ui.calc.buf = ""
		default:
			ui.calc.buf += msg.String()
		}
	}
	return ui, nil
}

func (ui UI) View() string {
	var s string
	for _, f := range ui.calc.stack {
		s += fmt.Sprintf(" %.*g\n", sigDigs, f)
	}
	s += boxTop(ui.windowWidth) + "\n"
	s += fmt.Sprintf("%[1]c%-*s%[1]c\n", boxVertical, ui.windowWidth-2, ui.calc.buf)
	s += boxBottom(ui.windowWidth)
	return s
}

// boxTop returns the top of a UTF-8 box, 'width' characters wide (including
// corners).
func boxTop(width int) string {
	if width < 1 {
		return ""
	}
	row := make([]rune, width)
	row[0] = boxTopLeft
	row[width-1] = boxTopRight
	if width > 1 {
		fill(row[1:width-1], boxHorizontal)
	}
	return string(row)
}

// boxBottom returns the botom of a UTF-8 box, 'width' characters wide
// (including corners).
func boxBottom(width int) string {
	if width < 1 {
		return ""
	}
	row := make([]rune, width)
	row[0] = boxBottomLeft
	row[width-1] = boxBottomRight
	if width > 1 {
		fill(row[1:width-1], boxHorizontal)
	}
	return string(row)
}

// fill fills s with c.
func fill(s []rune, c rune) {
	for i := range s {
		s[i] = c
	}
}
