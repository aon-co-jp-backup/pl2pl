<?php
class RequireToken {
    public function handle(Request $request, Closure $next) {
        if (!$request->header('X-Token')) {
            return response()->json(['error' => 'no token'], 401);
        }
        if ($request->header('X-Token') !== 'secret') abort(403);
        return $next($request);
    }
}
