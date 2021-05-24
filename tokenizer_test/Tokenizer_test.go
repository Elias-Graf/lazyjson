package tokenizer_test

import (
	"lazyJson/tokenizer"
	"reflect"
	"testing"
)

func TestKeywordLiterals(t *testing.T) {
	tests := []string{"false", "null", "true"}

	for _, test := range tests {
		t.Run(test, func(t *testing.T) {
			toks, err := tokenizer.Tokenize(test)
			exp := []*tokenizer.Token{{Val: test, Typ: tokenizer.LitKey}}

			expNoErr(t, err)
			expEquals(t, toks, exp)
		})
	}
}
func TestNumberLiterals(t *testing.T) {
	tests := []string{"1", "15", "19.1", "18.17"}

	for _, test := range tests {
		t.Run(test, func(t *testing.T) {
			toks, err := tokenizer.Tokenize(test)
			exp := []*tokenizer.Token{{Val: test, Typ: tokenizer.LitNum}}

			expNoErr(t, err)
			expEquals(t, toks, exp)
		})
	}
}
func TestOperators(t *testing.T) {
	tests := []string{":"}

	for _, test := range tests {
		t.Run(test, func(t *testing.T) {
			toks, err := tokenizer.Tokenize(test)
			exp := []*tokenizer.Token{{Val: test, Typ: tokenizer.Oper}}

			expNoErr(t, err)
			expEquals(t, toks, exp)
		})
	}
}
func TestSeparators(t *testing.T) {
	tests := []string{",", "[", "]", "{", "}"}

	for _, test := range tests {
		t.Run(test, func(t *testing.T) {
			toks, err := tokenizer.Tokenize(test)
			exp := []*tokenizer.Token{{Val: test, Typ: tokenizer.Sep}}

			expNoErr(t, err)
			expEquals(t, toks, exp)
		})
	}
}
func TestStringLiterals(t *testing.T) {
	tests := []string{"\"\"", "\"hello world\"", "\"hey \\\" newline!\""}

	for _, test := range tests {
		t.Run(test, func(t *testing.T) {
			toks, err := tokenizer.Tokenize(test)
			exp := []*tokenizer.Token{{
				Val: test[1 : len(test)-1],
				Typ: tokenizer.LitStr,
			}}

			expNoErr(t, err)
			expEquals(t, toks, exp)
		})
	}
}
func TestSkipWhitespace(t *testing.T) {
	tests := []string{" 0", "\t0", "\n0"}

	for _, test := range tests {
		t.Run(test, func(t *testing.T) {
			toks, err := tokenizer.Tokenize(test)
			exp := []*tokenizer.Token{{Val: "0", Typ: tokenizer.LitNum}}

			expNoErr(t, err)
			expEquals(t, toks, exp)
		})
	}
}
func TestIntegration(t *testing.T) {
	_, err := tokenizer.Tokenize(
		"[{\"key\": {\"values\": [true, false, null, 5000]}}]",
	)

	expNoErr(t, err)

}

func expEquals(t *testing.T, a interface{}, b interface{}) {
	if !reflect.DeepEqual(a, b) {
		t.Fatalf("Expected %s to equal %s", a, b)
	}
}
func expNoErr(t *testing.T, err error) {
	if err != nil {
		t.Fatal(err)
	}
}
