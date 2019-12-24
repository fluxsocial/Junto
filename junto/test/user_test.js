const { Orchestrator, Config } = require('@holochain/tryorama');
const scenarios = require("./scenarios.js");

const dnaJunto = Config.dna('./dist/junto.dna.json', 'junto');

const mainConfig = Config.gen(
    {
      junto: dnaJunto,  // agent_id="blog", instance_id="blog", dna=dnaBlog
    },
    {
        // specify a bridges
        bridges: [],
        logger: {
            type: 'debug',
            state_dump: false,
            rules: {
                rules: [{ exclude: true, pattern: ".*" }]
            }
        },
        // use a sim2h network
        network: {
            type: 'sim2h',
            sim2h_url: 'wss://sim2h.holochain.org:9000',
        },
    }
);
  
const orchestrator = new Orchestrator();

orchestrator.registerScenario('Can register a profile and retrieve', async (s, t) => {
    const {user1} = await s.players({user1: mainConfig}, true);
    let user1_res = await scenarios.registerAgent(t, user1, "jdeepee", "joshua", "parkin").catch(err => { console.log(err) } );
    await s.consistency();

    const get_username_from_address = await user1.call('junto', 'user', 'get_username_from_address', {username_address: user1_res.Ok.username.address})
    console.log("Getting user by address", get_username_from_address) //should return username
    t.equal(JSON.stringify(get_username_from_address), JSON.stringify({ Ok: { username: 'jdeepee' } }))
    console.log("Completed get username by address\n\n\n")

    const get_user_profile_from_address = await user1.call('junto', 'user', 'get_user_profile_from_address', {username_address: user1_res.Ok.username.address})
    console.log("Getting user profile by address", get_user_profile_from_address) //should return profile
    t.equal(JSON.stringify(get_user_profile_from_address), JSON.stringify({Ok: user1_res.Ok.profile}))
    console.log("Completed get profile by address\n\n\n")

    const get_user_profile_by_agent_address = await user1.call('junto', 'user', 'get_user_profile_by_agent_address', {})
    console.log("Get user profile", get_user_profile_by_agent_address) //should return profile
    t.equal(JSON.stringify(get_user_profile_by_agent_address), JSON.stringify({Ok: user1_res.Ok.profile}));
    console.log("Completed get profile by agent address\n\n\n")
  
    const get_user_username_by_agent_address = await user1.call('junto', 'user', 'get_user_username_by_agent_address', {})
    console.log("Get user username", get_user_username_by_agent_address) //should return username
    t.equal(JSON.stringify(get_user_username_by_agent_address), JSON.stringify({Ok: user1_res.Ok.username}));
    console.log("Completed get username by agent address\n\n\n")

    const get_user_data_by_agent_address = await user1.call('junto', 'user', 'get_user_data_by_agent_address', {})
    console.log("Get user data", get_user_data_by_agent_address) //should return username
    t.equal(JSON.stringify(get_user_data_by_agent_address), JSON.stringify(user1_res));
    console.log("Completed get username by agent address\n\n\n")
});

const report = orchestrator.run()
console.log(report)
