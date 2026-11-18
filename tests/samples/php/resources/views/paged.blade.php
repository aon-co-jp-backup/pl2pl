<h1>Total {{ $users->total() }}</h1>
@foreach($users as $u)
<p class="u">{{ $u['name'] }}</p>
@endforeach
{{ $users->links() }}
