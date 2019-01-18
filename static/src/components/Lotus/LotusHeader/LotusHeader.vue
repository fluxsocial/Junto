<template>
    <div class="lotusHeader">
        <div class="lotusHeader__content">
            <slot name="expressionType"></slot>
            <div class="lotusHeader__create">
                <div class="lotusHeader__create--channels">add channels</div>
                <slot name="lotusCreate"></slot>
            </div>

        </div>
    </div>
</template>
