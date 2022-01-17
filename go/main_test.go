package main

import (
	"bytes"
	"testing"
)

func TestInterpreter(t *testing.T) {
	input := []byte("+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.")
	c := newCompiler(input)
	c.compile()
	buf := &bytes.Buffer{}
	v := newVM(c.code)
	err := v.run(0, buf)
	if err != nil {
		t.Fatal(err)
	}

	expected := "hello world"
	if buf.String() != expected {
		t.Fatalf("expected='%s', got='%s'", expected, buf.String())
	}
}
