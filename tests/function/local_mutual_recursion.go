{
  fn isEven(n:int) -> bool {
    if (n == 0) return true;
    // out: NameError: name "isOdd" is not defined
    return isOdd(n - 1);
  }

  fn isOdd(n: int) -> bool {
    if (n == 0) return false;
    return isEven(n - 1);
  }

  isEven(4);
}
