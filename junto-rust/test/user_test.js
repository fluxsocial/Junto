const { Config, Container, Scenario } = require("@holochain/holochain-nodejs")
//const n3h = require('n3h');
Scenario.setTape(require('tape'))

const dnaPath = "./dist/junto-rust.dna.json"

const dna = Config.dna(dnaPath)
const agentJosh = Config.agent("josh")
const instanceJosh = Config.instance(agentJosh, dna)
const scenario = new Scenario([instanceJosh], {debugLog: true}) 

scenario.runTape('Can register a profile and retrieve', async (t, {josh}) => {
  const register_result = await josh.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}})
  console.log("Register user result", register_result)
  t.equal(JSON.stringify(register_result), JSON.stringify({"Ok":{"private_den":{"address":"QmRhbdLQupJsE4NZajLCR2oCpCZjncoP656bh5TwXBTyHi","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Private"}},"shared_den":{"address":"Qmb3U3NGDvzr9H74yiXq1LZEwx5V5qrCivVWuk5jJhr4Mf","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Shared"}},"public_den":{"address":"Qmf4LcJ77idWGMPeGN1ngoqnmwot8tNmSHZ3mF1dZC8xsp","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Public"}},"pack":{"address":"QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC","entry":{"name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}},"profile":{"address":"QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","entry":{"username":"jdeepee"}},"user_perspective":{"address":"QmaAhrUjfAKVSoZRs6VUjEk3WWuzEXCTqQozWRFS4Au4mz","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Default Perspective"}}}}));
  console.log("Completed register profile\n\n\n")

  const get_username_from_address = await josh.callSync('core', 'get_username_from_address', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'})
  console.log("Getting user by address", get_username_from_address) //should return username
  t.equal(JSON.stringify(get_username_from_address), JSON.stringify({ Ok: { username: 'jdeepee' } }))
  console.log("Completed get username by address\n\n\n")

  const get_user_profile_from_address = await josh.callSync('core', 'get_user_profile_from_address', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'})
  console.log("Getting user profile by address", get_user_profile_from_address) //should return profile
  t.equal(JSON.stringify(get_user_profile_from_address), JSON.stringify({ Ok: {"address": "QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1", "entry": {"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}} }))
  console.log("Completed get profile by address\n\n\n")

  const get_user_profile_by_agent_address = await josh.callSync('core', 'get_user_profile_by_agent_address', {})
  console.log("Get user profile", get_user_profile_by_agent_address) //should return profile
  t.equal(JSON.stringify(get_user_profile_by_agent_address), JSON.stringify({ Ok: {"address": "QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1", "entry": {"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}} }))
  console.log("Completed get profile by agent address\n\n\n")

  const get_user_username_by_agent_address = await josh.callSync('core', 'get_user_username_by_agent_address', {})
  console.log("Get user username", get_user_username_by_agent_address) //should return username
  t.equal(JSON.stringify(get_user_username_by_agent_address), JSON.stringify({ Ok: {"address": "QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn", "entry": {"username":"jdeepee"}} }))
  console.log("Completed get username by agent address\n\n\n")
})