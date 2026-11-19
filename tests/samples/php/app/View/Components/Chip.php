<?php
namespace App\View\Components;
class Chip extends Component {
    public function __construct(public string $tone = 'calm', public $size = 3) {}
    public function render() { return view('components.chip'); }
}
