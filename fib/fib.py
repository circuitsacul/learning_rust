from time import perf_counter


def fib(n):
    curr = 1
    last = 1

    if n in {1, 2}:
        return 1

    for _ in range(n-2):
        result = curr + last
        last = curr
        curr = result

    return curr


print(fib(500_000))
print(perf_counter())
