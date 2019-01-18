<template>
    <div class="canvas">
        <slot name="hellos"></slot> 
        <slot name="canvasSideHeader"></slot>
        <slot name="canvasSideMain"></slot>
        <slot name="canvasSearch"></slot>

        <div class="canvas__main">
            <slot name="canvasFeed"></slot>

        </div>
    </div>
</template>
