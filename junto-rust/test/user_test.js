const { Config, Container, Scenario } = require("@holochain/holochain-nodejs")
//const n3h = require('n3h');
Scenario.setTape(require('tape'))

const dnaPath = "./dist/junto-rust.dna.json"

const dna = Config.dna(dnaPath)
const agentJosh = Config.agent("josh")
const instanceJosh = Config.instance(agentJosh, dna)
const scenario = new Scenario([instanceJosh]) 

scenario.runTape('Can register a profile and retrieve', async (t, {josh}) => {
  const register_result = await josh.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}})
  console.log("Register user result", register_result)
  t.equal(JSON.stringify(register_result), JSON.stringify({ Ok: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn' }))
  console.log("Completed register profile\n\n\n")

  const get_username_from_address = await josh.callSync('core', 'get_username_from_address', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'})
  console.log("Getting user by address", get_username_from_address) //should return username
  t.equal(JSON.stringify(get_username_from_address), JSON.stringify({ Ok: { username: 'jdeepee' } }))
  console.log("Completed get username by address\n\n\n")

  const get_user_profile_from_address = await josh.callSync('core', 'get_user_profile_from_address', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'})
  console.log("Getting user profile by address", get_user_profile_from_address) //should return profile
  t.equal(JSON.stringify(get_user_profile_from_address), JSON.stringify({ Ok: 
    { parent: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn',
      first_name: 'Josh',
      last_name: 'Parkin',
      bio: 'Junto Testing',
      profile_picture: 'pictureurl',
      verified: true } }))
  console.log("Completed get profile by address\n\n\n")

  const get_user_profile_by_agent_address = await josh.callSync('core', 'get_user_profile_by_agent_address', {})
  console.log("Get user profile", get_user_profile_by_agent_address) //should return profile
  t.equal(JSON.stringify(get_user_profile_by_agent_address), JSON.stringify({ Ok: 
    { parent: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn',
      first_name: 'Josh',
      last_name: 'Parkin',
      bio: 'Junto Testing',
      profile_picture: 'pictureurl',
      verified: true } }))
  console.log("Completed get profile by agent address\n\n\n")

  const get_user_profile_address_by_agent_address = await josh.callSync('core', 'get_user_profile_address_by_agent_address', {})
  console.log("Get user profile address", get_user_profile_address_by_agent_address) //should return address
  t.equal(JSON.stringify(get_user_profile_address_by_agent_address), JSON.stringify({ Ok: 'QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1' }))
  console.log("Completed get profile address by agent address\n\n\n")

  const get_user_username_by_agent_address = await josh.callSync('core', 'get_user_username_by_agent_address', {})
  console.log("Get user username", get_user_username_by_agent_address) //should return username
  t.equal(JSON.stringify(get_user_username_by_agent_address), JSON.stringify({ Ok: { username: 'jdeepee' } }))
  console.log("Completed get username by agent address\n\n\n")

  const get_user_username_address_by_agent_address = await josh.callSync('core', 'get_user_username_address_by_agent_address', {})
  console.log("Get username address", get_user_username_address_by_agent_address) //should return username address 
  t.equal(JSON.stringify(get_user_username_address_by_agent_address), JSON.stringify({ Ok: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn' }))
  console.log("Completed get username address by agent address\n\n\n")
})