import {
  makeHolochainCall,
  isSuccess,
  makeHolochainCallAndReturn
} from "../../../utils.js";

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
        template.$store.commit("addUserHolochainData", result);
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

function getCurrentBitPrefix(template) {
  return makeHolochainCallAndReturn(
    template.$store.getters.getHolochainConnection,
    "config",
    "get_current_bit_prefix",
    {}
  )
    .then(result => {
      result = JSON.parse(result);
      if (isSuccess(result) == true) {
        console.log(
          "(getCurrentBitPrefix) great success on getting current bit prefix: ",
          result
        );
        return result;
      } else {
        console.log(
          "(getCurrentBitPrefix) Error on getting current bit prefix: ",
          result
        );
        template.$notify({
          type: "error",
          group: "main",
          title: "There was an error retrieving current bit prefix. Error is: ",
          text: result.Err.Internal,
          duration: 5000
        });
      }
    })
    .catch(err => {
      console.log("It failed", err);
    });
}

function updateCurrentBitPrefix(template, bitPrefix) {
  return makeHolochainCallAndReturn(
    template.$store.getters.getHolochainConnection,
    "config",
    "update_bit_prefix",
    { bit_prefix: bitPrefix }
  )
    .then(result => {
      result = JSON.parse(result);
      if (isSuccess(result) == true) {
        console.log(
          "(updateCurrentBitPrefix) great success on update bit prefix: ",
          result
        );
        return result;
      } else {
        console.log(
          "(updateCurrentBitPrefix) Error on update bit prefix: ",
          result
        );
        template.$notify({
          type: "error",
          group: "main",
          title: "There was an updating bit prefix. Error is: ",
          text: result.Err.Internal,
          duration: 5000
        });
      }
    })
    .catch(err => {
      console.log("It failed", err);
    });
}

export default {
  registerUser,
  getCurrentBitPrefix,
  updateCurrentBitPrefix
};
