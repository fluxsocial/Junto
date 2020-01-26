<template>
  <section class="profile">
    <div class="profile--card-container">
      <div class="profile--card">
        <div class="profile--card-content-left">
          <div
            ref="avatarEditorContainer"
            class="avatar-editor-container"
            style="width: 100%; height: 100%; display: none;"
          >
            <VueAvatar
              ref="avatarEditor"
              :rotation="rotation"
              :scale="scale"
              :border-radius="borderRadius"
              class="avatar-editor"
              @vue-avatar-editor:image-ready="onImageReady"
            />
            <Button
              :method="saveProfilePicture"
              text="Save"
              active-class="test-button-here"
            />
          </div>
          <img
            ref="profilePicture"
            :src="profile.entry.profile_picture"
            class="profile-picture"
            @click="editProfilePicture()"
          />
        </div>
        <div class="profile--card-content-right">
          <div class="profile--edit">
            <svg class="edit-icon" @click="editProfile()">
              <use
                xlink:href="../../../src/assets/img/sprite.svg#icon-edit"
              ></use>
            </svg>
          </div>
          <div class="profile--info">
            <p class="profile--info-name">
              {{ profile.entry.first_name }} {{ profile.entry.last_name }}
            </p>
            <p class="profile--info-username">@{{ username.entry.username }}</p>
            <p class="profile--info-bio">{{ profile.entry.bio }}</p>
          </div>
          <div class="profile--stats">
            <div class="profile--stats-item stats-expressions">
              <h6>Expressions</h6>
              <span>44</span>
            </div>
            <div class="profile--stats-item stats-groups">
              <h6>Groups</h6>
              <span>2</span>
            </div>
            <div class="profile--stats-item stats-packs">
              <h6>Pack</h6>
              <span>
                <router-link :to="{ name: 'pack' }">{{
                  this.packName
                }}</router-link>
              </span>
            </div>
          </div>
          <div class="profile--links">
            <Button
              :method="routeTo"
              text="Public Den"
              active-class="profile--links-btn-container"
              url="publicDen"
            />
            <Button
              :method="routeTo"
              text="Private Den"
              active-class="profile--links-btn-container"
              url="privateDen"
            />
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script>
import profileHttpMethods from "./ProfileHttp";
import Button from "../../Button/Button";
import { VueAvatar } from "vue-avatar-editor-improved";
import Cookies from "js-cookie";

export default {
  name: "Profile",
  components: {
    VueAvatar,
    Button
  },
  props: {
    username: Object,
    profile: Object,
    address: String
  },
  data() {
    return {
      rotation: 0,
      scale: 1,
      borderRadius: 150,
      packName: String
    };
  },
  mounted() {
    this.renderPackData();
    console.log("renderingComponentData now: ", this.packName);
  },
  methods: {
    onImageReady() {
      console.log("inside onImageReady");
    },
    editProfile() {
      console.log("Editing profile info");
    },
    editProfilePicture() {
      console.log("Editing profile picture");
      const avatarEditorContainer = this.$refs.avatarEditorContainer;
      const avatarEditor = this.$refs.avatarEditor;
      const profilePictureEl = this.$refs.profilePicture;
      avatarEditorContainer.style.display = "flex";
      profilePictureEl.style.display = "none";
      avatarEditor.clicked();
    },
    saveProfilePicture() {
      console.log("saving profile picture");
      let img = this.$refs.avatarEditor.getImageScaled();
      const profilePictureEl = this.$refs.profilePicture;
      profilePictureEl.src = img.toDataURL();
      profilePictureEl.style.display = "block";

      const avatarEditorContainer = this.$refs.avatarEditorContainer;
      avatarEditorContainer.style.display = "none";
    },
    routeTo(toUrl) {
      console.log("routing to... ", toUrl);
      this.$router.push({ name: toUrl });
    },
    renderPackData() {
      this.packName = Cookies.getJSON("cookieStore").userPack.entry.name;
    }
  }
};
</script>
