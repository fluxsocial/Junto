import { makeHolochainCall, isSuccess } from "./../../utils";

function getUserProfileByAgentAddress(template) {
    makeHolochainCall(
        template.$store.getters.getHolochainConnection,
        "user",
        "get_user_data_by_agent_address",
        {},
        result => {
            if (isSuccess(result) == true) {
                console.log("great success on getting user profile: ", result);
                template.$store.commit("addUserHolochainData", result);
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
    );
}

function getUserProfileByUsernameAddress(template, target_address) {
    makeHolochainCall(
        template.$store.getters.getHolochainConnection,
        "user",
        "get_user_data_by_agent_address",
        {username_address: target_address},
        result => {
            if (isSuccess(result) == true) {
                console.log("great success on getting user profile: ", result);
                return result;
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
                    template.$router.push("/");
                }, 5000);
            }
        }
    );
}

export default {getUserProfileByAgentAddress, getUserProfileByUsernameAddress};
