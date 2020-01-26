import { isSuccess, makeHolochainCallAndReturn } from "./../../utils";

function createExpression(template, expression_data, dna_address, channels) {
  return makeHolochainCallAndReturn(
    template.$store.getters.getHolochainConnection,
    "expression",
    "post_expression",
    {
      expression: expression_data,
      context: [dna_address],
      attributes: channels
    }
  )
    .then(result => {
      result = JSON.parse(result);
      if (isSuccess(result) == true) {
        console.log(
          "(createExpression) great success on posting expression: ",
          result
        );
        return result;
      } else {
        console.log(
          "(createExpression) Error on getting posting expression: ",
          result
        );
        template.$notify({
          type: "error",
          group: "main",
          title:
            "There was an error retrieving this posting expression. Error is: ",
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
  createExpression
};
