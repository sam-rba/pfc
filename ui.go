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
const sigDigs = 17

type UI struct {
	calc   Calculator
	width  int // Width of the window measured in characters.
	height int
}

func (ui UI) Init() tea.Cmd {
	return nil
}

func (ui UI) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		ui.width = msg.Width
		ui.height = msg.Height
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "Q":
			return ui, tea.Quit
		case "J", "K":
			ui.calc.swap()
		case "D":
			ui.calc.buf = ""
		case "C":
			ui.calc.buf = ""
			ui.calc.stack = ui.calc.stack[:0]
		case "A":
			ui.calc.anglem = !ui.calc.anglem
		case "N":
			ui.calc.negate()
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
				fn(ui.calc)
			} else if con := parseConstant(ui.calc.buf); con != nil {
				ui.calc.stack.push(*con)
			} else if f, err := strconv.ParseFloat(ui.calc.buf, 64); err == nil {
				ui.calc.stack.push(f)
			}
			ui.calc.buf = ""
		default:
			ui.calc.buf += msg.String()
		}
	}
	return ui, nil
}

func (ui UI) View() string {
	s := padding(ui)

	// Angle mode.
	s += fmt.Sprintf("%*s\n", ui.width-1, ui.calc.anglem)

	// Stack.
	top := boxTop(ui.width)
	bottom := boxBottom(ui.width)
	s += top + "\n"
	for _, f := range ui.calc.stack {
		s += fmt.Sprintf("%[1]c%*s%[1]c\n", boxVertical, ui.width-2, printNum(f))
	}
	s += bottom + "\n"

	// Buffer.
	s += boxTop(ui.width) + "\n"
	s += fmt.Sprintf("%[1]c%*s%[1]c\n", boxVertical, ui.width-2, ui.calc.buf)
	s += boxBottom(ui.width)
	return s
}

func padding(ui UI) string {
	var ( // Number of lines occupied by each ui element.
		anglem = 1
		stack  = len(ui.calc.stack) + 2
		buf    = 3
	)
	padlines := ui.height - anglem - stack - buf
	if padlines < 1 {
		return ""
	}
	s := make([]byte, padlines)
	for i := 0; i < padlines; i++ {
		s[i] = '\n'
	}
	return string(s)
}

func printNum(v float64) string {
	return fmt.Sprintf(" %.*g", sigDigs, v)
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
