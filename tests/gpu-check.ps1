# pl2pl gpu の出力(CUDA 形式のカーネル)を、実際の GPU(GeForce GT 730 = sm_35)で動かして確かめる(Windows、管理者権限は不要)。
# 前提: Visual Studio 2022 Build Tools(MSVC)。CUDA 11.8 の nvcc は micromamba で $Work\env に入れる(NVIDIA の conda チャンネル)。
# 使い方: powershell -File tests\gpu-check.ps1 [-Work F:\_work\cuda118] [-Pl2pl path\to\pl2pl.exe]
# 注意: ドライバ 475(CUDA 11.4 相当)なので CUDA 11.x まで。sm_35 は 11.8 で「非推奨」警告つきで使える(12 系は不可)。
param([string]$Work = "F:\_work\cuda118", [string]$Pl2pl = "pl2pl.exe")
$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
New-Item -ItemType Directory -Force $Work | Out-Null
if (-not (Test-Path "$Work\env\bin\nvcc.exe")) {
  if (-not (Test-Path "$Work\micromamba.exe")) {
    Invoke-WebRequest -Uri https://github.com/mamba-org/micromamba-releases/releases/latest/download/micromamba-win-64 -OutFile "$Work\micromamba.exe" -UseBasicParsing
  }
  & "$Work\micromamba.exe" create -y -q -p "$Work\env" -c nvidia/label/cuda-11.8.0 cuda-nvcc cuda-cudart-dev cuda-cudart
}
$out = "$Work\gpu-out"; Remove-Item -Recurse -Force $out -ErrorAction SilentlyContinue
& $Pl2pl gpu "$root\tests\samples\hpc\vec.cpp" --out $out --target open-cuda | Out-Null
@'
#include <cstdio>
#include <cuda_runtime.h>
#include "vec_loop12.cu"
int main() {
    const int N = 1 << 20; size_t sz = N * sizeof(float);
    float *a = new float[N], *b = new float[N], *c = new float[N];
    for (int i = 0; i < N; i++) { a[i] = (float)i; b[i] = 0.5f * i; }
    float *da, *db, *dc; cudaMalloc(&da, sz); cudaMalloc(&db, sz); cudaMalloc(&dc, sz);
    cudaMemcpy(da, a, sz, cudaMemcpyHostToDevice); cudaMemcpy(db, b, sz, cudaMemcpyHostToDevice);
    int n = N, lo = 0;
    pl2pl_kernel_12<<<(n + 255) / 256, 256>>>(da, db, dc, n, lo);
    if (cudaDeviceSynchronize() != cudaSuccess) { printf("kernel error\n"); return 3; }
    cudaMemcpy(c, dc, sz, cudaMemcpyDeviceToHost);
    int bad = 0; for (int i = 0; i < N; i++) if (c[i] != a[i] + b[i]) bad++;
    printf("generated kernel on GPU: mismatches=%d\n", bad);
    return bad != 0;
}
'@ | Set-Content "$out\harness.cu" -Encoding ascii
$bat = "$out\b.bat"
@"
@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
set PATH=$Work\env\bin;%PATH%
cd /d $out
nvcc -D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH -arch=sm_35 -allow-unsupported-compiler -Wno-deprecated-gpu-targets -I$Work\env\include -L$Work\env\lib harness.cu -o harness.exe -lcudart
"@ | Set-Content $bat -Encoding ascii
cmd /c $bat | Out-Null
$env:PATH = "$Work\env\bin;$env:PATH"
& "$out\harness.exe"
if ($LASTEXITCODE -eq 0) { "gpu: OK" } else { "gpu: FAILED (exit $LASTEXITCODE)"; exit 1 }
