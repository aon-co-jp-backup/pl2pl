#include <stdio.h>
#include <stdlib.h>
#define N 1000000

static double a[N], b[N], c[N];

int main(void) {
    for (int i = 0; i < N; i++) {
        a[i] = i * 0.5;
        b[i] = 2.0 * i;
    }
    for (int i = 0; i < N; i++) {
        c[i] = a[i] + b[i];
    }
    double sum = 0.0;
    for (int i = 0; i < N; i++) {
        sum += c[i] * 0.001;
    }
    for (int i = 1; i < N; i++) {
        a[i] = a[i - 1] + 1.0;
    }
    printf("sum=%.3f a_last=%.1f\n", sum, a[N - 1]);
    return 0;
}
