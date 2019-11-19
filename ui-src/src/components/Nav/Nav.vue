
<template>
   <div class="navigation">
       <div class="navigation__content">
            <div class="navigation__top">
                <router-link to="/" class="navigation__top--den">
                    <svg class="navigation__top--icon">
                        <use xlink:href="../../../src/assets/img/sprite.svg#icon-square"></use>
                     </svg>
                </router-link>


                <router-link to="/pack" class="navigation__top--pack">
                    <svg class="navigation__top--icon">
                        <use xlink:href="../../../src/assets/img/sprite.svg#icon-triangle"></use>
                     </svg>
                </router-link>
                <router-link to="/collective" class="navigation__top--collective">
                    <svg class="navigation__top--icon">
                        <use xlink:href="../../../src/assets/img/sprite.svg#icon-circle"></use>
                     </svg>    
                </router-link>
                <slot name="navigationLogo"></slot>

                <svg class="navigation__top--notifs">
                    <use xlink:href="../../../src/assets/img/sprite.svg#icon-moon"></use>
                 </svg>    
                
            </div>
            <slot name='navigationBorder'></slot>
            <!-- <slot name="navigationBottom"></slot> -->
       </div>

   </div>
 

</template>

<script>
    export default {
        data() {
            return {

            }
        }
    }
</script>


<style>


</style>
