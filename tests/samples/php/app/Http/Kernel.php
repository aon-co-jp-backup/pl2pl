<?php
class Kernel {
    protected $routeMiddleware = [
        'token' => \App\Http\Middleware\RequireToken::class,
        'localonly' => \App\Http\Middleware\LocalOnly::class,
    ];
}
