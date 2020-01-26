<template>
  <section class="register">
    <div class="register__content">
      <div class="register__left"></div>

      <form
        class="register__form"
        enctype="multipart/form-data"
        @submit.prevent
      >
        <div class="register__form__child">
          <h3>Welcome to <span>Junto</span></h3>
          <p>
            Register here! You will be automagically logged in when you return.
          </p>
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
            <div class="file-upload-preview-img-container">
              <img :src="previewImg" class="preview-img" />
            </div>
            <input
              id="profile-picture"
              type="file"
              accept="image/*"
              class="form-control"
              placeholder="Upload a profile picture"
              @change="processFile($event)"
            />
            <label id="profile-picture-label" for="profile-picture"
              >Drag your image here <br />
              or click to browse</label
            >
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
        <Button
          :method="registerHttp"
          text="Join Junto"
          active-class="register__submit register__form__child"
        />
      </form>
    </div>
  </section>
</template>

<script>
import registerUser from "./RegisterHttp.js";
import Button from "../../Button/Button";

export default {
  name: "Register",
  components: {
    Button
  },
  data() {
    return {
      userData: {
        username: "",
        first_name: "",
        last_name: "",
        profile_picture: "",
        bio: ""
      },
      previewImg: null
    };
  },
  methods: {
    registerHttp(event) {
      registerUser(this, this.userData);
    },
    processFile(event) {
      if (
        !event.target.files[0] ||
        event.target.files[0].type.startsWith("image/") == false
      ) {
        this.$notify({
          type: "error",
          group: "main",
          title: "Incorrect image format",
          text: "Accepted formats: jpg, jpeg, png, svg",
          duration: 3000
        });
      } else {
        const label = event.target.labels[0]; //image upload label inside dashed box
        label.className = "uploading";
        label.innerHTML =
          'Uploaded image: <span class="image-name">' +
          JSON.stringify(
            // Simple HTML entity encoding for printed image name
            String(event.target.files[0].name)
              .replace(/&/g, "&amp;")
              .replace(/</g, "&lt;")
              .replace(/>/g, "&gt;")
              .replace(/"/g, "&quot;")
              .replace(/'/g, "&#x27;")
          ) +
          "</span>";

        const image = event.target.files[0]; //show uploaded image as a preview
        const reader = new FileReader();
        reader.readAsDataURL(image);
        reader.onload = e => {
          //e is a different event
          this.previewImg = e.target.result;
          this.userData.profile_picture = e.target.result; //image as base64
        };
        event.target.previousElementSibling.style.display = "flex";
      }
    }
  }
};
</script>
