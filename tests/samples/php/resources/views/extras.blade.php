@extends('layouts.stackdemo')
@section('content')
<p>n={{ $n }}</p>
@switch($n)
@case(1)
<b>one</b>
@break
@case(2)
<b>two</b>
@break
@default
<b>many</b>
@endswitch
@auth
<i>in</i>
@endauth
@guest
<i>out</i>
@endguest
<input @checked($n == 2)>
<script>var d = @json($data);</script>
@endsection
@push('scripts')
<script src="/x.js"></script>
@endpush
