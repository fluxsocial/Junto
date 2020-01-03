import { makeHolochainCall, isSuccess } from "./../../utils";

function getUserProfile(template) {
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

export default getUserProfile;
