fn f() -> fn {
  for (let i = 1; i < 2; i = i + 1) {
    let i = "i";
    fn g() { println i; }
    return g;
  }
}

let h = f();
h(); // out: i
