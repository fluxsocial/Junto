// component for profile from member's point of view

<template>
    <section class="den">
        <junto-nav>
            <img slot="navigationLogo" class="navigation__top--logo" src="./../../assets/img/junto-web__logo--grey.png">
            <!-- <div slot="navigationBottom" class="navigation__bottom navigation__bottom--den">
                <p class="navigation__bottom--space">DEN</p>
            </div> -->
            <div class="navigation__border navigation__border--den" slot="navigationBorder"></div>

        </junto-nav>     

        <div class="den__content">
            <div class="den__backdrop">
                <div class="den__backdrop--details">
                    <p class="den__backdrop--name">Eric Yang</p>
                    <p class="den__backdrop--handle">@sunyata </p>
                    <div class="den__backdrop--shortbio">
                        <p class="den__backdrop--shortbio--text">
                            "to a mind that is still, the whole universe surrenders" 
                        </p>
                    </div>
                </div>
            </div>

            <div class="den__body">
                <div class="den__canvas">
                    <div class="den__canvas--nav">
                        <p class="den__canvas--nav--item">EXPRESSIONS</p>
                        <p class="den__canvas--nav--item">PACK</p>
                        <p class="den__canvas--nav--item">JOURNAL</p>
                        <p class="den__canvas--nav--item">ABOUT</p>
                    </div>
                    <div class="den__canvas--body">
                        

                    </div>

                </div>

                <div class="den__profile">
                    <div class="den__profile--picture">
                        <img src="./../../assets/img/junto-web__eric.png" alt="" class="den__profile--picture--photo">
                    </div>
                    <!-- <p class="den__profile--name">Eric Yang</p> -->
                    <!-- <p class="den__profile--handle">@sunyata</p> -->
                    
                </div>
            </div>

        </div>


        <junto-lotus>
            <svg slot="lotusIcon" class="lotus__icon lotus__icon--den">
                <use xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon"></use>
            </svg>
        </junto-lotus>
    </section> 
</template>

<script>
import Nav from './../Nav/Nav.vue'
import Lotus from './../Lotus/Lotus.vue'

    export default {
        components: {
            juntoNav: Nav,
            juntoLotus: Lotus,
        }
        
    }
</script>
