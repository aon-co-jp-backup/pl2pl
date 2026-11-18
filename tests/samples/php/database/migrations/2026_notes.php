<?php
Schema::create('notes', function (Blueprint $table) {
    $table->id();
    $table->string('text');
});
