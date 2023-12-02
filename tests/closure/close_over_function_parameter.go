let f;

fn foo(param: String) {
  fn f_() {
    println param;
  }
  f = f_;
}
foo("param");

f(); // out: param
