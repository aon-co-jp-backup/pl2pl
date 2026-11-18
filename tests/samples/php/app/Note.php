<?php
class Note extends Model {
    public function noteable() { return $this->morphTo(); }
}
