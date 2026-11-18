<?php
use Illuminate\Support\Facades\Route;

Route::get('/', function () { return 'home'; });
Route::get('/users/{id}', [UserController::class, 'show']);
Route::post('/users', [UserController::class, 'store']);
Route::prefix('api')->group(function () {
    Route::get('/ping', fn() => ['ok' => true, 'v' => 1]);
    Route::apiResource('items', ItemController::class);
});
Route::get('/health', [UserController::class, 'health']);
Route::get('/echo/{id}', [UserController::class, 'echoId']);
