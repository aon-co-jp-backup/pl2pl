<?php
Schema::create('users', function (Blueprint $table) {
    $table->id();
    $table->string('name');
    $table->integer('age');
    $table->timestamps();
});
