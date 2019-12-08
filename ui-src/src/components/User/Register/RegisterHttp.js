import { makeHolochainCall, isSuccess } from "../../../utils.js";

function registerUser(
  template,
  username,
  first_name,
  last_name,
  profile_picture,
  bio
) {
  makeHolochainCall(
    template.$store.getters.getHolochainConnection,
    "user",
    "create_user",
    {
      username: username,
      first_name: first_name,
      last_name: last_name,
      profile_picture: profile_picture,
      bio: bio
    },
    result => {
      if (isSuccess(result) == true) {
        console.log("User has registered here is the result: ", result);
        result;
      } else {
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
