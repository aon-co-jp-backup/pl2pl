@props(['type' => 'info'])
<div class="alert {{ $type }}"><h4>{{ $title }}</h4>{{ $slot }}</div>
