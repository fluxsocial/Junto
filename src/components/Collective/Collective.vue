
<template>
    <section class="collective"> 
        <!-- nav -->
        <junto-nav>
            <img slot="navigationLogo" class="navigation__top--logo" src="./../../assets/img/junto-web__logo--blue.png">
            <!-- <div slot="navigationBottom" class="navigation__bottom navigation__bottom--collective">
                <p class="navigation__bottom--space">JUNTO</p>
            </div> -->
            <div class="navigation__border navigation__border--collective" slot="navigationBorder"></div>

        </junto-nav>    

        <junto-sidebar-collective></junto-sidebar-collective>

        <!-- collective canvas -->
        <junto-canvas>

            <!-- search -->
            <div class="canvas__search" slot="canvasSearch">
                <input type="text" class="canvas__search--text" placeholder="search channels">
            </div>

            <!-- feed -->
            <div class="canvas__feed" slot="canvasFeed">

                <!-- story expression -->
                <div class="canvas__expression canvas__story">
                    <!-- expression top -->
                    <div class="canvas__expression--top">
                        <div class="canvas__expression--profile">
                            <button class="canvas__expression--profile--picture">&nbsp;</button>

                            <div class="canvas__expression--profile--details">
                                <p class="canvas__expression--profile--name">Eric Yang</p>
                                <p class="canvas__expression--profile--handle">@sunyata</p>
                            </div>
                        </div>
                        <p class="canvas__expression--edit">edit</p>
                    </div>

                    <!-- expression body -->
                    <p class="canvas__story--title">The Medium is the Message</p>
                    <p class="canvas__story--body">Hi. Before we start, I should let you know we’re going to be intentionally vague about our specific product features (for now). The purpose of this article is to shed light on our core design philosophy, why the current..</p>

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

                <!-- story expression -->
                <div class="canvas__expression canvas__photo">
                    <!-- expression top -->
                    <div class="canvas__expression--top">
                        <div class="canvas__expression--profile">
                            <button class="canvas__expression--profile--picture">&nbsp;</button>

                            <div class="canvas__expression--profile--details">
                                <p class="canvas__expression--profile--name">Eric Yang</p>
                                <p class="canvas__expression--profile--handle">@sunyata</p>
                            </div>
                        </div>
                        <p class="canvas__expression--edit">edit</p>
                    </div>

                    <!-- expression story -->

                    <img src="./../../assets/img/junto-web__sacred.png" alt="" class="canvas__photo--photo">
                    <p class="canvas__photo--caption">Livin</p>

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


                <!-- story expression -->
                <div class="canvas__expression canvas__story">
                    <!-- expression top -->
                    <div class="canvas__expression--top">
                        <div class="canvas__expression--profile">
                            <button class="canvas__expression--profile--picture">&nbsp;</button>

                            <div class="canvas__expression--profile--details">
                                <p class="canvas__expression--profile--name">Eric Yang</p>
                                <p class="canvas__expression--profile--handle">@sunyata</p>
                            </div>
                        </div>
                        <p class="canvas__expression--edit">edit</p>
                    </div>

                    <!-- expression story -->
                    <p class="canvas__story--title">The Medium is the Message</p>
                    <p class="canvas__story--body">Hi. Before we start, I should let you know we’re going to be intentionally vague about our specific product features (for now). The purpose of this article is to shed light on our core design philosophy, why the current..</p>

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
            </div>
        </junto-canvas>


        <!-- create -->
        <junto-lotus>
            <svg slot="lotusIcon" class="lotus__icon lotus__icon--collective">
                <use xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon"></use>
            </svg>
        </junto-lotus>

    </section>
</template>


<script>
import Lotus from './../Lotus/Lotus.vue'
import Nav from './../Nav/Nav.vue'
import Canvas from './../Canvas/Canvas.vue'
import SidebarCollective from './../Sidebar/SidebarCollective/SidebarCollective.vue'

    export default {
        components: {
            juntoNav: Nav,
            juntoLotus: Lotus,
            juntoCanvas: Canvas,
            juntoSidebarCollective: SidebarCollective
        }
        
    }
</script>
 