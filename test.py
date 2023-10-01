import time


def fib_recursive(n):
    return 1 if n < 2 else fib_recursive(n - 2) + fib_recursive(n - 1)


start = time.time()
print(fib_recursive(35))
print(time.time() - start)
