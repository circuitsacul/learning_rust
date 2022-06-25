from time import perf_counter

N: int = 10_000_000

x: list[int] = list(range(N))
for idx, i in enumerate(x):
    x[idx] = i + 1

print(perf_counter())
