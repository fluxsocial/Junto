<template>
  <section class="register">
    <div class="register__content">
      <div class="register__left"></div>

      <form @submit.prevent class="register__form">
        <div class="register__form__child">
          <h3>Welcome to <span>Junto</span></h3>
          <p>Register here! You will be automagically logged in when you return.</p>
        </div>
        <div class="register__form__child">
          <div class="form-group">
            <input
              id="firstName"
              v-model="userData.first_name"
              type="text"
              class="form-control"
              placeholder="First Name"
            />
            <label for="firstName">First Name:</label>
          </div>
          <div class="form-group">
            <input
              id="lastName"
              v-model="userData.last_name"
              type="text"
              class="form-control"
              placeholder="Last Name"
            />
            <label for="lastName">Last Name:</label>
          </div>
          <div class="form-group">
            <input
              id="username"
              v-model="userData.username"
              type="text"
              class="form-control"
              placeholder="Username"
            />
            <label for="username">Username:</label>
          </div>
          <div class="form-group">
            <input
              id="profile-picture"
              v-model="userData.profile_picture"
              type="text"
              class="form-control"
              placeholder="Upload a profile picture"
            />
            <label for="profile-picture">Upload a profile picture</label>
          </div>
          <div class="form-group">
            <textarea
              id="bio"
              v-model="userData.bio"
              type="text"
              class="form-control"
              placeholder="Write something about yourself"
            />
            <label for="bio">Write something about yourself:</label>
          </div>
        </div>
        <div class="register__submit register__form__child">
          <button class="btn register__submit__btn" @click="registerHttp()">Join Junto</button>
        </div>
      </form>
    </div>
  </section>
</template>

<script>
import registerUser from "./RegisterHttp.js";

export default {
  name: "Register",
  components: {},
  data() {
    return {
      userData: {
        username: "",
        first_name: "",
        last_name: "",
        profile_picture: "",
        bio: ""
      }
    };
  },
  methods: {
    registerHttp(event) {
      registerUser(this, this.userData);
    }
  }
};
</script>
