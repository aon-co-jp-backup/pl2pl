<?php
class Comment extends Model {
    public function post() { return $this->belongsTo(Post::class); }
}
