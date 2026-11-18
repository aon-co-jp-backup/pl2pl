<?php
class UserController {
    public function show(int $id) { return User::find($id); }
    public function store(Request $request) { return User::create($request->all()); }
    public function health() { return ['status' => 'ok']; }
    public function echoId(int $id) { return ['id' => $id]; }
    public function destroy($id) { return User::destroy($id); }
}
