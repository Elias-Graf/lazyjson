package tokenizer

func operator(inp string, idx int) (tok *Token, cons int, err error) {
	if inp[idx] != ':' {
		return
	}
	tok = &Token{":", Oper}
	cons = 1
	return
}
