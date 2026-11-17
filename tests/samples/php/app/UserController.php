<?php
class UserController {
    public function show(int $id) { return User::find($id); }
    public function store(Request $request) { return User::create($request->all()); }
}
