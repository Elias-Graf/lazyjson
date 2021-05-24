package tokenizer

var symbols = map[rune]bool{
	',': true, '[': true, ']': true, '{': true, '}': true,
}

func separator(inp string, idx int) (*Token, int, error) {
	if len(inp) > idx && symbols[rune(inp[idx])] {
		return &Token{Val: inp, Typ: Sep}, 1, nil
	}
	return nil, 0, nil
}
