// fn outer() -> fn {
//   let x = "value";
//   fn middle() -> fn {
//     fn inner() {
//       print x;
//     }
//
//     println "create inner closure";
//     return inner;
//   }
//
//   println "return from outer";
//   return middle;
// }
//
// let mid = outer();
// let in = mid();
// in();

// {
//   let a = 3;
//   fn f() {
//     fn g() {
//         println a;
//     }
//     return g();
//   }
//
//     f();
// }
//

fn outer() -> fn {
  let a = 1;
  let b = 2;
  fn middle() -> fn {
    let c = 3;
    let d = 4;
    fn inner() {
      print a + c + b + d;
    }
    return inner;
  }
    return middle;
}

let mid = outer();
let in = mid();
in();
