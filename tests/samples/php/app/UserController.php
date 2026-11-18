<?php
class UserController {
    public function show(int $id) { return User::find($id); }
    public function store(Request $request) { return User::create($request->all()); }
    public function health() { return ['status' => 'ok']; }
    public function echoId(int $id) { return ['id' => $id]; }
    public function destroy($id) { return User::destroy($id); }
    public function update(Request $request, $id) {
        $user = User::findOrFail($id);
        $user->update($request->all());
        return response()->json($user);
    }
    public function rename($id) {
        $user = User::findOrFail($id);
        $user->name = 'renamed';
        $user->save();
        return $user;
    }
    public function remove($id) {
        $user = User::findOrFail($id);
        $user->delete();
        return response()->noContent();
    }
    public function make(Request $request) {
        $user = User::create($request->all());
        return response()->json($user, 201);
    }
}
