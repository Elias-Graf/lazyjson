package treebuilder_test

import (
	"lazyJson/tokenizer"
	"lazyJson/treebuilder"
	"testing"
)

func TestArrBuild_Empty(t *testing.T) {
	cons := treebuilder.ArrBuild([]tokenizer.Token{})

	if cons != 0 {
		t.Fatalf("Expected to consume '0', consumed '%v'", cons)
	}
}
func TestArrBuild_NotArray(t *testing.T) {
	cons := treebuilder.ArrBuild([]tokenizer.Token{
		{Val: "{", Typ: tokenizer.Punct}})

	if cons != 0 {
		t.Fatalf("Expected to consume '0', consumed '%v'", cons)
	}
}
func TestArrBuild_EmptyArray(t *testing.T) {
	cons := treebuilder.ArrBuild([]tokenizer.Token{
		{Val: "[", Typ: tokenizer.Punct},
		{Val: "]", Typ: tokenizer.Punct},
	})

	if cons != 2 {
		t.Fatalf("Expected to consume '2', consumed '%v'", cons)
	}
}
