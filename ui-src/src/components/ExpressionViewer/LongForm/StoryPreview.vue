<template>
    <!-- story expression -->
    <div class="canvas__expression canvas__story">
        <!-- expression top -->
        <div class="canvas__expression--top">
        <div class="canvas__expression--profile">
            <button class="canvas__expression--profile--picture">&nbsp;</button>

            <div class="canvas__expression--profile--details">
            <p class="canvas__expression--profile--name">{{ users_name }}</p>
            <p class="canvas__expression--profile--handle">{{ username }}</p>
            </div>
        </div>
        </div>

        <!-- expression story -->
        <p class="canvas__story--title">{{ title }}</p>
        <p class="canvas__story--body">
            {{ body }}
        </p>

        <!-- expression bottom -->
        <div class="canvas__expression--bottom">
        <div class="canvas__expression--channels">
            <button class="canvas__expression--channel">design</button>
            <button class="canvas__expression--channel">philosophy</button>
        </div>

        <div class="canvas__expression--responses">
            <button class="canvas__expression--resonate">&nbsp;</button>
            <button class="canvas__expression--comment">&nbsp;</button>
        </div>
        </div>
    </div>
</template>