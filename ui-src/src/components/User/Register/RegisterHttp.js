import { makeHolochainCall, isSuccess } from "../../../utils.js";

function registerUser(template, userData) {
  makeHolochainCall(
    template.$store.getters.getHolochainConnection,
    "user",
    "create_user",
    {
      user_data: {
        username: userData.username,
        first_name: userData.first_name,
        last_name: userData.last_name,
        profile_picture: userData.profile_picture,
        bio: userData.bio
      }
    },
    result => {
      if (isSuccess(result) == true) {
        console.log("User has registered here is the result: ", result);
        result;
      } else {
        console.log(result);
        template.$notify({
          type: "error",
          group: "main",
          title: "There was an error creating the account. Error is: " + result,
          duration: 1000
        });
      }
    }
  );
}

export default registerUser;
