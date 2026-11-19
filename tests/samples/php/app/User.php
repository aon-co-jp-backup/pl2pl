<?php
class User extends Model {
    protected $fillable = ['name', 'age'];
    public function posts() { return $this->hasMany(Post::class); }
    public function labels() { return $this->morphToMany(Label::class, 'labelable'); }
    public function roles() { return $this->belongsToMany(Role::class); }
}
