package tokenizer

type TokenTyp string

const (
	Punct TokenTyp = "Punctuation"
	Str   TokenTyp = "String"
	Num   TokenTyp = "Number"
)
