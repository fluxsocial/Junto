const {Diorama, tapeExecutor} = require('@holochain/diorama')
const scenarios = require("./scenarios.js")
const dnaPath = Diorama.dna('./dist/junto-rust.dna.json', 'junto')

const diorama = new Diorama({
    instances: {
        agent1: dnaPath
    },
    debugLog: false,
    executor: tapeExecutor(require('tape'))
});

diorama.registerScenario('Can register a profile and retrieve', async (s, t, {agent1}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin").catch(err => { console.log(err) } );

    const get_username_from_address = await agent1.call('user', 'get_username_from_address', {username_address: user1.Ok.username.address})
    console.log("Getting user by address", get_username_from_address) //should return username
    t.equal(JSON.stringify(get_username_from_address), JSON.stringify({ Ok: { username: 'jdeepee' } }))
    console.log("Completed get username by address\n\n\n")
  
    const get_user_profile_from_address = await agent1.call('user', 'get_user_profile_from_address', {username_address: user1.Ok.username.address})
    console.log("Getting user profile by address", get_user_profile_from_address) //should return profile
    t.equal(JSON.stringify(get_user_profile_from_address), JSON.stringify({Ok: user1.Ok.profile}))
    console.log("Completed get profile by address\n\n\n")
  
    const get_user_profile_by_agent_address = await agent1.call('user', 'get_user_profile_by_agent_address', {})
    console.log("Get user profile", get_user_profile_by_agent_address) //should return profile
    t.equal(JSON.stringify(get_user_profile_by_agent_address), JSON.stringify({Ok: user1.Ok.profile}));
    console.log("Completed get profile by agent address\n\n\n")
  
    const get_user_username_by_agent_address = await agent1.call('user', 'get_user_username_by_agent_address', {})
    console.log("Get user username", get_user_username_by_agent_address) //should return username
    t.equal(JSON.stringify(get_user_username_by_agent_address), JSON.stringify({Ok: user1.Ok.username}));
    console.log("Completed get username by agent address\n\n\n")
});

diorama.run();