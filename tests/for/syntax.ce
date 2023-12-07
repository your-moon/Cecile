// Single-expression body.
//
// out: 1
// out: 2
// out: 3
//
for (let c = 0; c < 3;) println c = c + 1;

// Block body.
//
// out: 0
// out: 1
// out: 2
//
for (let a = 0; a < 3; a = a + 1) {
  println a;
}

// No clauses.
//
// out: done
//
fn foo() -> String {
  for (;;) return "done";
}
println foo();

// No variable.
//
// out: 0
// out: 1
//
let i = 0;
for (; i < 2; i = i + 1) println i;

// No condition.
//
// out: 0
// out: 1
// out: 2
//
fn bar() {
  for (let i = 0;; i = i + 1) {
    println i;
    if (i >= 2) return;
  }
}
bar();


// No increment.
//
// out: 0
// out: 1
//
for (let i = 0; i < 2;) {
  println i;
  i = i + 1;
}


// Statement bodies.
for (; false;) if (true) 1; else 2;
for (; false;) while (true) 1;
for (; false;) for (;;) 1;
