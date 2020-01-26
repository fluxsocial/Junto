// component for profile from member's point of view

<template>
  <section class="den">
    <div class="den__content">
      <div class="den__backdrop">
        <div class="den__backdrop--details">
          <p class="den__backdrop--name">
            {{ profile.entry.first_name }} {{ profile.entry.last_name }}
          </p>
          <p class="den__backdrop--handle">@{{ username.entry.username }}</p>
          <div class="den__backdrop--shortbio">
            <p class="den__backdrop--shortbio--text">
              "{{ profile.entry.bio }}"
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
          <div class="den__canvas--body"></div>
        </div>

        <div class="den__profile">
          <div class="den__profile--picture">
            <img
              :src="profile.entry.profile_picture"
              alt="Profile Picture"
              class="den__profile--picture--photo"
            />
          </div>
          <!-- <p class="den__profile--name">Eric Yang</p> -->
          <!-- <p class="den__profile--handle">@sunyata</p> -->
        </div>
      </div>
    </div>

    <junto-lotus>
      <svg slot="lotusIcon" class="lotus__icon lotus__icon--den">
        <use
          xlink:href="../../../src/assets/img/sprite.svg#icon-lotusicon"
        ></use>
      </svg>
    </junto-lotus>
  </section>
</template>

<script>
import Lotus from "./../../Lotus/Lotus.vue";

export default {
  components: {
    juntoLotus: Lotus
  },
  props: {
    //See structure of username & profile props in User.vue data()
    username: Object,
    profile: Object
  }
};
</script>
