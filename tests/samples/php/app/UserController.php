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
    public function adults() {
        return User::where('age', '>=', 18)->orderBy('name')->get();
    }
    public function named(Request $request) {
        $user = User::where('name', 'bob')->first();
        return $user;
    }
    public function many() {
        return User::where('age', '<', 18)->orWhere('name', 'zed')->latest()->limit(2)->get();
    }
    public function total() {
        $n = User::where('age', '>', 0)->count();
        return response()->json(['n' => $n]);
    }
    public function tagged() {
        return User::where('tag', 'vip')->get();
    }
}
