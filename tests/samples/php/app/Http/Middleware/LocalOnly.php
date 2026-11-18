<?php
class LocalOnly {
    public function handle(Request $request, Closure $next) {
        $response = $next($request);
        $response->header('X-Local', '1');
        return $response;
    }
}
