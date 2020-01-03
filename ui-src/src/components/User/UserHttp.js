import { makeHolochainCall, isSuccess } from "./../../utils";

function getUserProfile(template) {
  makeHolochainCall(
    template.$store.getters.getHolochainConnection,
    "user",
    "get_user_profile_by_agent_address",
    {},
    result => {
      if (isSuccess(result) == true) {
        console.log("great success on getting user profile: ", result);
        template.$store.commit("addProfileHolochainData", result);
        getUserUsername(template);
      } else {
        if (template.$route.path == "/user/register") {
          console.log(
            "Don't show $notify error on register page if user profile not found"
          );
          console.log("Error on getting user profile: ", result);
        } else {
          console.log("Error on getting user profile: ", result);
          template.$notify({
            type: "error",
            group: "main",
            title: "There was an error retrieving this user profile. Error is: ",
            text: result.Err.Internal,
            duration: 5000
          });
          setTimeout(function() {
            template.$router.push("/user/register");
          }, 5000);
        }
      }
    }
  );
}

function getUserUsername(template) {
  makeHolochainCall(
    template.$store.getters.getHolochainConnection,
    "user",
    "get_user_username_by_agent_address",
    {},
    result => {
      if (isSuccess(result) == true) {
        template.$store.commit("addUsernameHolochainData", result);
      } else {
        console.log("Username not found: ", result);
        template.$notify({
          type: "error",
          group: "main",
          title:
            "There was an error retrieving the Username for this profile: ",
          text: result.Err.Internal,
          duration: 5000
        });
      }
    }
  );
}

export default getUserProfile;
