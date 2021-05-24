package tokenizer

import (
	"fmt"
)

type consumer = func(string, int) (*Token, int, error)

var consumers = []consumer{
	skipWhitespace,
	keywordLiteral,
	numberLiteral,
	operator,
	separator,
	stringLiteral,
}

func Tokenize(input string) (toks []*Token, err error) {
	consumed := 0

	for consumed < len(input) {
		prev := consumed

		for _, cons := range consumers {
			t, c, e := cons(input, consumed)

			if e != nil {
				err = e
				return
			}

			consumed += c
			if t != nil {
				toks = append(toks, t)
			}

			if consumed == len(input) {
				break
			}
		}

		if consumed == prev {
			err = fmt.Errorf(
				"no one consumed \"%s\" at index %d",
				string(input[consumed]),
				consumed,
			)
			return
		}
	}

	if len(input) != consumed {
		toks = nil
		err = fmt.Errorf(
			"failed to consume the whole input string, exit at: %d",
			consumed,
		)

		return
	}

	return
}
