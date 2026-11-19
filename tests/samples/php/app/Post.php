<?php
class Post extends Model {
    public function user() { return $this->belongsTo(User::class); }
    public function comments() { return $this->hasMany(Comment::class); }
    public function labels() { return $this->morphToMany(Label::class, 'labelable'); }
    public function tags() { return $this->morphMany(Note::class, 'noteable'); }
}
