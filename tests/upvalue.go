fn outer() -> fn {
  let a = 1;
  let b = 2;
  fn middle() -> fn {
    let c = 3;
    let d = 4;
    fn inner() {
      println a + c + b + d;
    }
    return inner;
  }
    return middle;
}

let mid = outer();
let in = mid();
in(); // out: 10
