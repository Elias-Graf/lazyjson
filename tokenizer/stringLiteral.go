package tokenizer

import "fmt"

func stringLiteral(inp string, idx int) (tok *Token, cons int, err error) {
	if isNotLongEnoughToBeString(inp, idx) {
		return
	}
	if inp[idx] != '"' {
		return
	}

	cons++
	val := ""

	for i := cons; idx+i < len(inp); i++ {
		if isEndOfString(inp, idx+i) {
			cons++
			break
		} else if isNotEndOfStringButEndOfInput(inp, idx+i) {
			err = fmt.Errorf("unterminated string")
			return
		}

		val += string(inp[idx+i])
		cons++
	}

	if cons == 0 {
		return
	}

	tok = &Token{val, LitStr}
	return
}

func isNotLongEnoughToBeString(inp string, idx int) bool {
	return len(inp) < idx+1
}
func isEndOfString(inp string, idx int) bool {
	return inp[idx] == '"' && inp[idx-1] != '\\'
}
func isNotEndOfStringButEndOfInput(inp string, idx int) bool {
	return !isEndOfString(inp, idx) && idx > len(inp)-1
}
