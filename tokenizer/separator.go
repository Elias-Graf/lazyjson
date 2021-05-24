package tokenizer

var symbols = map[rune]bool{
	',': true, '[': true, ']': true, '{': true, '}': true,
}

func separator(inp string, idx int) (*Token, int, error) {
	if symbols[rune(inp[idx])] {
		return &Token{Val: string(inp[idx]), Typ: Sep}, 1, nil
	}
	return nil, 0, nil
}
