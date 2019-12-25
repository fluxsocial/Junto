<template>
    <div class="nav-dropdown-container">
        <svg class="navigation__top--icon" >
            <use v-bind:xlink:href='"../../../src/assets/img/sprite.svg#" + Icon'></use>
        </svg>
        <div class="dropdown">
            <ul class="dropdown__list">
                <router-link tag="li" v-for="item in Items" :key="item.text" class="dropdown__list--item" :to="item.url">
                    {{ item.text }}
                </router-link>
            </ul>
        </div>
    </div>
</template>

<script>
export default {
    name: "Dropdown",
    props: {
        Items: {
            type: Array,
            required: true
        },
        Icon: {
            type: String,
            required: true
        }
    },
    data() {
        return {}
    }
}
</script>