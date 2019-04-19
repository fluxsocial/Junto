const {Config, Container, Scenario} = require("@holochain/holochain-nodejs")
Scenario.setTape(require('tape'))

const dnaPath = "./dist/junto-rust.dna.json"

const dna = Config.dna(dnaPath)
const agentJosh = Config.agent("josh")
const instanceJosh = Config.instance(agentJosh, dna)
const scenario = new Scenario([instanceJosh], {debugLog: true})

scenario.runTape('Can register profile and retrieve', async (t, {josh}) =>{
  const register_result = await josh.callSync("core", "create_user", {user_data: {first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}})
  console.log(register_result)
})