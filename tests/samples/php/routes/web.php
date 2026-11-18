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
Route::delete('/users/{id}', [UserController::class, 'destroy']);
Route::put('/users/{id}', [UserController::class, 'update']);
Route::post('/users/{id}/rename', [UserController::class, 'rename']);
Route::delete('/users/{id}/remove', [UserController::class, 'remove']);
Route::post('/make', [UserController::class, 'make']);
Route::get('/adults', [UserController::class, 'adults']);
Route::get('/named', [UserController::class, 'named']);
Route::get('/many', [UserController::class, 'many']);
Route::get('/total', [UserController::class, 'total']);
Route::get('/tagged', [UserController::class, 'tagged']);
Route::get('/people', [UserController::class, 'people']);
Route::get('/people-compact', [UserController::class, 'peopleCompact']);
Route::get('/people-with', [UserController::class, 'peopleWith']);
Route::view('/about', 'about');
Route::get('/welcome', function () { return view('about'); });
Route::get('/secure', function () { return ['secure' => true]; })->middleware('token');
Route::middleware(['auth', 'throttle:2,1'])->prefix('private')->group(function () {
    Route::get('/me', function () { return ['me' => 1]; });
});
Route::get('/after', function () { return 'x'; })->middleware('localonly');
Route::get('/guestonly', function () { return 'g'; })->middleware('guest');
