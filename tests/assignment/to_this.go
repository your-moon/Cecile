type Foo;
impl Foo {
  fn foo() {
    self = "value"; // out: SyntaxError: unexpected "="
  }
}

Foo();

