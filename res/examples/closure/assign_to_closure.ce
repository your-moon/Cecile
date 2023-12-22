let f;
let g;

{
  let local = "local";
  fn f_() {
    println local;
    local = "after f";
    println local;
  }
  f = f_;

  fn g_() {
    println local;
    local = "after g";
    println local;
  }
  g = g_;
}

f();
// out: local
// out: after f

g();
// out: after f
// out: after g

