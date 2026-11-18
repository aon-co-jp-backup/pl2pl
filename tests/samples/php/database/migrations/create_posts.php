<?php
Schema::create('posts', function (Blueprint $table) {
    $table->id();
    $table->foreignId('user_id');
    $table->string('title');
    $table->boolean('published');
});
Schema::create('roles', function (Blueprint $table) {
    $table->id();
    $table->string('name');
});
