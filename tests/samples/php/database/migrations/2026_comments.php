<?php
Schema::create('comments', function (Blueprint $table) {
    $table->id();
    $table->foreignId('post_id');
    $table->string('body');
});
