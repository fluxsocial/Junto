import { isSuccess, makeHolochainCallAndReturn } from "./../../utils";

function getExpression(
  template,
  perspective,
  attributes,
  queryOptions,
  targetType,
  queryType,
  dos,
  seed,
  resonations
) {
  return makeHolochainCallAndReturn(
    template.$store.getters.getHolochainConnection,
    "expression",
    "query_expressions",
    {
      perspective: perspective,
      attributes: attributes,
      query_options: queryOptions,
      target_type: targetType,
      query_type: queryType,
      dos: dos,
      seed: seed,
      resonations: resonations
    }
  )
    .then(result => {
      result = JSON.parse(result);
      if (isSuccess(result) == true) {
        console.log("Great Success getting expressions", result);
      } else {
        console.log("Error getting expressions: ", result);
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
      console.log("getExpression Failed: ", err);
    });
}

export default {
  getExpression
};
