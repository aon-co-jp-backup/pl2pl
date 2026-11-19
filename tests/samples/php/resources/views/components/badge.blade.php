@props(['kind' => 'info'])
<span{{ $attributes->merge(['class' => 'badge']) }}>{{ $kind }}:{{ $slot }}</span>
