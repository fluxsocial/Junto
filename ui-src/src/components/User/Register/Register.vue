<template>
  <section class="register">
    <div class="register__content">
      <div class="register__left"></div>

      <form class="register__form" @submit.prevent enctype="multipart/form-data">
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
          <div class="form-group file-upload">
            <div class="file-upload-header">
              <p>Upload a profile picture</p>
            </div>
            <input
              id="profile-picture"
              @change="processFile($event)"
              type="file"
              accept="image/*"
              class="form-control"
              placeholder="Upload a profile picture"
            />
            <label id="profile-picture-label" for="profile-picture">Drag your image here <br> or click to browse</label>
          </div>
          <div class="form-group">
            <textarea
              id="bio"
              v-model="userData.bio"
              type="text"
              class="form-control"
              placeholder="Write something for your bio"
            />
            <label for="bio">Write something for your bio:</label>
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
      console.log(this.userData);
    },
    processFile(event){
      if(!event.target.files[0] || event.target.files[0].type.startsWith('image/') == false ){
        this.$notify({
          type: "error",
          group: "main",
          title: "Incorrect image format",
          text: "Accepted formats: jpg, jpeg, png, svg",
          duration: 3000
        });
      }else{
        console.log(event);
        event.target.labels[0].innerHTML = 'Uploaded image: <span class="image-name">' + event.target.files[0].name + '</span>';
        this.userData.profile_picture = event.target.files[0];  //userData.profile_picture expects a string.
      } 
    }
  }
};
</script>
