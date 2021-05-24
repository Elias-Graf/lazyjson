package tokenizer

import "unicode"

func skipWhitespace(inp string, idx int) (tok *Token, cons int, err error) {
	if unicode.IsSpace(rune(inp[idx])) {
		cons++

		if idx+cons < len(inp) {
			_, c, _ := skipWhitespace(inp, idx+cons)

			cons += c
		}
	}

	return
}
