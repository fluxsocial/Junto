<template>
    
    <div class="lotusBullet">
        <!-- <input type="text" class="lotusBullet__title" placeholder="Title (optional)" maxlength="255"> -->
        <button @click="createBullet">hello</button>
        <div class="lotusBullet__cards">
            <div class="lotusBullet__card">
                <textarea name="" id="" cols="30" rows="10" class="lotusBullet__card--text lotusBullet__card--text-title" placeholder="Title (optional)"></textarea>
            </div>

            <div class="lotusBullet__card lotusBullet__card--1">
                <p class="lotusBullet__card--number">1/{{ bulletCount }}</p>
                <textarea name="" id="" cols="30" rows="10" class="lotusBullet__card--text"></textarea>
            </div>
            <!-- <div class="lotusBullet__card">
                <textarea name="" id="" cols="30" rows="10" class="lotusBullet__card--text"></textarea>
            </div> -->

        </div>

    </div>
</template>

<script>
    export default {
        data() {
            return {
                bulletCount: 1,
                currentCount: 1

            }
        },

        methods: {
            createBullet() {
             let newBulletCount = this.bulletCount++;
             this.currentCount++;

             let newBullet = `
                    <div class="lotusBullet__card lotusBullet__card--${newBulletCount}">
                        <p class="lotusBullet__card--number"> </p>
                        <textarea name="" id="" cols="30" rows="10" class="lotusBullet__card--text"></textarea>
                    </div>             
                `

            document.querySelector('.lotusBullet__cards').insertAdjacentHTML('beforeend', newBullet);
            }
        }
        
    }
</script>
