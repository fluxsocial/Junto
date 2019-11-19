<template>
    <div class="lotusShortform">
        <div class="lotusShortform__backgrounds">
            <button class="lotusShortform__background" :class="{ 'backgroundActive': backgrounds[0].active }" @click="backgroundOne">&nbsp;</button>
            <button class="lotusShortform__background" :class="{ 'backgroundActive': backgrounds[1].active }" @click="backgroundTwo">&nbsp;</button>
            <button class="lotusShortform__background" :class="{ 'backgroundActive': backgrounds[2].active }" @click="backgroundThree">&nbsp;</button>
            <button class="lotusShortform__background" :class="{ 'backgroundActive': backgrounds[3].active  }" @click="backgroundFour">&nbsp;</button>
            <button class="lotusShortform__background" :class="{ 'backgroundActive': backgrounds[4].active  }" @click="backgroundFive">&nbsp;</button>
            <button class="lotusShortform__background" :class="{ 'backgroundActive': backgrounds[5].active  }" @click="backgroundSix">&nbsp;</button>
        </div>
        <div class="lotusShortform__canvas">
            <textarea name="" id="" cols="30" rows="10" class="lotusShortform__canvas--text"></textarea>
        </div>
    </div>
</template> 

<script>
    export default {
        data() {
            return {
                hi: true,
                backgrounds: [
                    { active: true },
                    { active: false },
                    { active: false },
                    { active: false },
                    { active: false },
                    { active: false }
                ]
            }
        },

        methods: {
            backgroundOne() {
                this.backgrounds.forEach(function(background, index) {
                    background.active = false;
                })

                this.backgrounds[0].active = true;
                console.log(this.backgrounds[0].active);


            },

            backgroundTwo() {
                this.backgrounds.forEach(function(background, index) {
                    background.active = false;
                })

                this.backgrounds[1].active = true;
            },

            backgroundThree() {
                this.backgrounds.forEach(function(background, index) {
                    background.active = false;
                })

                this.backgrounds[2].active = true;
            },

            backgroundFour() {
                this.backgrounds.forEach(function(background, index) {
                    background.active = false;
                })

                this.backgrounds[3].active = true;
            },

            backgroundFive() {
                this.backgrounds.forEach(function(background, index) {
                    background.active = false;
                })

                this.backgrounds[4].active = true;
            },

            backgroundSix() {
                this.backgrounds.forEach(function(background, index) {
                    background.active = false;
                })

                this.backgrounds[5].active = true;
            },
        }
    }
</script>


<style lang="scss">
    .lotusShortform__canvas {
        background-image: linear-gradient(to right, rgba(0,0,0,.7), rgba(0,0,0,.7)), url(./../../../../assets/img/junto-web__logo--blue.png);
        background-size: cover;
        background-position: center;
        background-repeat: no-repeat;
    }

    .backgroundActive {
        border: 2px solid #A46BCC; 
    }
</style>
