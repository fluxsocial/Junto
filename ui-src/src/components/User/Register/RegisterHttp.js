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
        console.log("Success on register: ", result);
        template.$store.commit('addUserHolochainData', result);
        console.log("The state is now: ", template.$store.getters.getState);
        template.$router.push("/");
      } else {
        console.log("Error on registration: ", result);
        template.$notify({
          type: "error",
          group: "main",
          title: "There was an error creating the account. Error is: ",
          text: result.Err,
          duration: 10000
        });
      }
    }
  );
}

export default registerUser;
