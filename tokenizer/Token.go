package tokenizer

import "fmt"

type Token struct {
	Val string
	Typ TokenTyp
}

func (t Token) String() string {
	return fmt.Sprintf("Token[%s]{\"%s\"}", t.Typ, t.Val)
}
