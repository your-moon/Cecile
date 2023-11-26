type Point;

impl Point {}

fn test() -> fn {
  fn test_2() {
  }
  return test_2;
}

let nil_c = nil;
let bool_ = true;
let number = 1;
let string = "string";
let object = Point();
let func = test();
let uninitialized;

println func;
println object;
println string;
println number;
println bool_;
