<template>
  <div class="lotusHeader">
    <div class="lotusHeader__content">
      <slot name="expressionType"></slot>
      <div class="lotusHeader__create">
        <slot name="lotusChannels"></slot>
        <slot name="lotusCreate"></slot>
      </div>
    </div>
  </div>
</template>
