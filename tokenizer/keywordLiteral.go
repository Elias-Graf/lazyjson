package tokenizer

var keywords = []string{"false", "null", "true"}

func keywordLiteral(inp string, idx int) (*Token, int, error) {
	for _, keyword := range keywords {
		kLen := len(keyword)

		if len(inp) >= idx+kLen && inp[idx:idx+kLen] == keyword {
			return &Token{Val: keyword, Typ: LitKey}, kLen, nil
		}
	}
	return nil, 0, nil
}
