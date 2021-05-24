package tokenizer

type TokenTyp string

const (
	LitKey TokenTyp = "KeywordLiteral"
	LitNum TokenTyp = "NumberLiteral"
	LitStr TokenTyp = "StringLiteral"
	Oper   TokenTyp = "Operator"
	Sep    TokenTyp = "Separator"
)
