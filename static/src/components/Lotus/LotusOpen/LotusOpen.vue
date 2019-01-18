<template>
    <div class="lotus__container">
        <div class="lotus__post">
            <junto-lotus-header>
                <p slot="expressionType" class="lotusHeader__expressionType"> {{ expressionType }}</p>
                <button slot="lotusCreate" class="lotusHeader__create--button" @click="createExpression">create</button>

            </junto-lotus-header>
            
            <junto-lotus-story v-if="storyOpen"></junto-lotus-story>
            <junto-lotus-shortform v-if="shortformOpen"></junto-lotus-shortform>
            <junto-lotus-bullet v-if="bulletOpen"></junto-lotus-bullet>
            <junto-lotus-photo v-if="photoOpen"></junto-lotus-photo>
            <junto-lotus-photo v-if="eventsOpen"></junto-lotus-photo>
            <junto-lotus-photo v-if="musicOpen"></junto-lotus-photo>

        </div>
         <junto-lotus-footer>
            <svg slot="lotus" class="lotusFooter__lotus" :class="{'storyActive': storyOpen, 
                                                                    'shortformActive': shortformOpen, 
                                                                    }" @click="closeLotus">
                <use xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon"></use>
            </svg>
            <button slot="lotusStory" class="lotusFooter__expression" @click="openStory" v-bind:class="{ 'lotusFooter__expression--story': storyOpen }"></button>
            <button slot="lotusShortform" class="lotusFooter__expression" @click="openShortform" v-bind:class="{ 'lotusFooter__expression--shortform': shortformOpen }"></button>
            <button slot="lotusBullet" class="lotusFooter__expression" @click="openBullet">&nbsp;</button>
            <button slot="lotusPhoto" class="lotusFooter__expression" @click="openPhoto">&nbsp;</button>
            <button slot="lotusEvents" class="lotusFooter__expression" @click="openEvents">&nbsp;</button>
            <button slot="lotusMusic" class="lotusFooter__expression" @click="openMusic">&nbsp;</button>

        </junto-lotus-footer> 


    </div>

</template>

<script>
    import VueFroala from 'vue-froala-wysiwyg';
    import LotusHeader from './../LotusHeader/LotusHeader.vue'
    import LotusFooter from './../LotusFooter/LotusFooter.vue'
    import LotusStory from './../LotusExpressions/LotusStory/LotusStory.vue'
    import LotusShortform from './../LotusExpressions/LotusShortform/LotusShortform.vue'
    import LotusBullet from './../LotusExpressions/LotusBullet/LotusBullet.vue'
    import LotusPhoto from './../LotusExpressions/LotusPhoto/LotusPhoto.vue' 
    import LotusEvents from './../LotusExpressions/LotusEvents/LotusEvents.vue' 
    import LotusMusic from './../LotusExpressions/LotusMusic/LotusMusic.vue' 

    export default {
        data() {
            return {
                storyOpen: true,
                shortformOpen: false,
                bulletOpen: false,
                photoOpen: false,
                eventsOpen: false,
                musicOpen: false,
                expressionType: 'STORY',
            }
        },

        components: {
            juntoLotusHeader: LotusHeader,
            juntoLotusFooter: LotusFooter,
            juntoLotusStory: LotusStory,
            juntoLotusShortform: LotusShortform,
            juntoLotusBullet: LotusBullet,
            juntoLotusPhoto: LotusPhoto,
        },


        methods: {
            createExpression() {
                console.log('I created an expression !');
            },

            closeLotus() {
                this.$router.go(-1);
            },
        
            openStory() {
                this.storyOpen = true;
                this.shortformOpen = false;
                this.bulletOpen = false;
                this.photoOpen = false; 
                this.eventsOpen = false;
                this.musicOpen = false;
                this.expressionType = 'STORY';
            },

            openShortform() {
                this.storyOpen = false;
                this.shortformOpen = true;
                this.bulletOpen = false;
                this.photoOpen = false; 
                this.eventsOpen = false;
                this.musicOpen = false;
                this.expressionType = 'SHORTFORM';
            },

            openBullet() {
                this.storyOpen = false;
                this.shortformOpen = false;
                this.bulletOpen = true;
                this.photoOpen = false; 
                this.eventsOpen = false;
                this.musicOpen = false;
                this.expressionType = 'BULLET';
            },

            openPhoto() {
                this.storyOpen = false;
                this.shortformOpen = false;
                this.bulletOpen = false;
                this.photoOpen = true; 
                this.eventsOpen = false;
                this.musicOpen = false;
                this.expressionType = 'PHOTO';
            },

            openEvents() {
                this.storyOpen = false;
                this.shortformOpen = false;
                this.bulletOpen = false;
                this.photoOpen = false; 
                this.eventsOpen = true;
                this.musicOpen = false;
                this.expressionType = 'EVENTS';

            },

            openMusic() {
                this.storyOpen = false;
                this.shortformOpen = false;
                this.bulletOpen = false;
                this.photoOpen = false; 
                this.eventsOpen = false;
                this.musicOpen = true;
                this.expressionType = 'MUSIC';
            }

        }
    }   
</script>


<style lang="scss">
    @import './../../../../sass/base';

    .storyActive {
        fill: $junto-primary;
    }

    .shortformActive {
        fill: $junto-secondary;
    }
</style>
