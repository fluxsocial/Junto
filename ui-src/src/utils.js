import { Settings } from "./settings.js";

export const makeHolochainCall = function makeHolochainCall(
  connection,
  zome,
  func,
  params,
  callback
) {
  return connection.then(({ callZome }) => {
    callZome(
      Settings.InstanceId,
      zome,
      func
    )(params).then(result => {
      callback(JSON.parse(result))
    })
    .catch(err => {
      console.log("It failed", err);
    });
  });
};

export const makeHolochainCallAndReturn = function makeHolochainCall(
  connection,
  zome,
  func,
  params
) {
  return connection.then(({ callZome }) => {
    return callZome(
      Settings.InstanceId,
      zome,
      func
    )(params)
  });
};

export const isSuccess = function isSuccess(data) {
  if ("Ok" in data) {
    return true;
  } else {
    return false;
  }
};
