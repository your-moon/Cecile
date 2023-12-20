fn isEven(n: int) -> bool {
  if (n == 0) return true;
  return isOdd(n - 1);
}

fn isOdd(n: int) -> bool {
  if (n == 0) return false;
  return isEven(n - 1);
}

print isEven(4); // out: true
print isOdd(3); // out: true
