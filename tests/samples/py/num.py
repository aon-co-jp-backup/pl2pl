import math


def dot(a, b, n):
    s = 0.0
    for i in range(n):
        s += a[i] * b[i]
    return s


def describe(x):
    print("value", x)
    return x


if __name__ == "__main__":
    n = 100000
    a = [float(i) for i in range(n)]
    b = [2.0 for _ in range(n)]
    print(round(dot(a, b, n), 1))
