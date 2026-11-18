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
    public function people() {
        $people = User::where('age', '>', 0)->get();
        return view('people', ['heading' => 'All', 'people' => $people]);
    }
    public function peopleCompact() {
        $heading = 'Compact';
        $people = [];
        return view('people', compact('heading', 'people'));
    }
    public function peopleWith() {
        return view('people')->with('heading', 'With')->with('people', []);
    }
    public function userPosts($id) {
        $user = User::findOrFail($id);
        return $user->posts;
    }
    public function addPost(Request $request, $id) {
        $user = User::findOrFail($id);
        $post = $user->posts()->create($request->all());
        return response()->json($post, 201);
    }
    public function author($id) {
        $post = Post::findOrFail($id);
        return $post->user;
    }
    public function withPosts() {
        return User::with('posts')->get();
    }
    public function publishedAuthors() {
        return User::whereHas('posts', function ($q) { $q->where('published', true); })->get();
    }
    public function giveRole(Request $request, $id) {
        $user = User::findOrFail($id);
        $user->roles()->attach($request->input('role_id'));
        return $user->roles;
    }
    public function paged() {
        return User::orderBy('id')->paginate(2);
    }
    public function pagedView(Request $request) {
        $users = User::orderBy('id')->paginate(2)->withQueryString();
        return view('paged', compact('users'));
    }
    public function skipped() {
        return User::orderBy('id')->offset(1)->limit(2)->get();
    }
    public function search(Request $request) {
        $q = $request->query('q', 'none');
        return ['q' => $q];
    }
}
