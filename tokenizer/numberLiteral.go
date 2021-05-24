package tokenizer

import (
	"unicode"
)

func numberLiteral(inp string, idx int) (tok *Token, cons int, err error) {
	val := ""
	hasDecimal := false

	for _, r := range inp[idx:] {
		if r == '.' {
			// Only allow one decimal point
			if hasDecimal {
				break
			}

			hasDecimal = true
			val += string(r)
		} else if !unicode.IsDigit(r) {
			break
		} else {
			val += string(r)
		}

		cons++
	}

	if cons == 0 {
		return
	}

	tok = &Token{val, LitNum}
	return
}
