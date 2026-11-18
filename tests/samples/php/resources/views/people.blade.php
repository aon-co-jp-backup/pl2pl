@extends('layouts.app')
@section('title', 'People')
@section('content')
<h1>{{ $heading }}</h1>
@forelse($people as $p)
<li>{{ $p['name'] }} ({{ $p['age'] }}) {!! $p['tag'] ?? '-' !!}</li>
@empty
<p>nobody</p>
@endforelse
@if(count($people) > 1)<small>many</small>@else<small>few</small>@endif
@endsection
