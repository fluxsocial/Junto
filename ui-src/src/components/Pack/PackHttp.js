import { isSuccess, makeHolochainCallAndReturn } from "./../../utils";

function getUsersPack(template, target_username_address) {
  return makeHolochainCallAndReturn(
    template.$store.getters.getHolochainConnection,
    "group",
    "get_user_pack",
    { username_address: target_username_address }
  )
    .then(result => {
      result = JSON.parse(result);
      if (isSuccess(result) == true) {
        console.log(
          "(getUsersPack) great success on getting user pack: ",
          result
        );
        template.$store.commit("addUserPackData", result);
        return result;
      } else {
        console.log("(getUsersPack) Error on getting user pack: ", result);
        template.$notify({
          type: "error",
          group: "main",
          title: "There was an error retrieving this users pack. Error is: ",
          text: result.Err.Internal,
          duration: 5000
        });
        setTimeout(function() {
          template.$router.push("/");
        }, 5000);
      }
    })
    .catch(err => {
      console.log("It failed", err);
    });
}

export default { getUsersPack };
