<template>
  <div class="test">
    <div class="tab-content">
      <div id="signup">
        <h1>Make movements</h1>

        <form @submit.prevent>
          <div class="row">
            <div
              class="col-xs-12 col-sm-8 col-sm-offset-2 col-md-6 col-md-offset-3"
            >
              <h1>Signup</h1>
              <hr />
              <div class="form-group">
                <label for="firstName">First Name</label>
                <input
                  id="firstName"
                  v-model="userData.first_name"
                  type="text"
                  class="form-control"
                />
              </div>
              <div class="form-group">
                <label for="lastName">Last Name</label>
                <input
                  id="lastName"
                  v-model="userData.last_name"
                  type="text"
                  class="form-control"
                />
              </div>
              <div class="form-group">
                <label for="username">Username</label>
                <input
                  id="username"
                  v-model="userData.username"
                  type="text"
                  class="form-control"
                />
              </div>
              <div class="form-group">
                <label for="profile-picture">Upload a profile picture</label>
                <input
                  id="profile-picture"
                  v-model="userData.profile_picture"
                  type="text"
                  class="form-control"
                />
              </div>
              <div class="form-group">
                <label for="bio">Write something about yourself</label>
                <textarea
                  id="bio"
                  v-model="userData.bio"
                  type="text"
                  class="form-control"
                />
              </div>
            </div>
          </div>
          <hr />
          <div class="row">
            <div
              class="col-xs-12 col-sm-8 col-sm-offset-2 col-md-6 col-md-offset-3"
            >
              <button class="btn btn-primary" @click="registerHttp()">
                Join Junto
              </button>
            </div>
          </div>
        </form>
      </div>
    </div>
  </div>
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
