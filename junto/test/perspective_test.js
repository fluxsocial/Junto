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

orchestrator.registerScenario('Can add and get users from perspective', async (s, t) => {
    const {agent1, agent2} = await s.players({agent1: mainConfig, agent2: mainConfig}, true);
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    await s.consistency();

    const add_user = await scenarios.addUserToPerspective(t, agent1, user1.Ok.user_perspective.address, user2.Ok.username.address);
    await s.consistency();
    const get_users = await scenarios.getPerspectivesUsers(t, agent1, user1.Ok.user_perspective.address);
    t.equal(JSON.stringify(get_users), JSON.stringify({"Ok":[user2.Ok.username]}));
});

const report = orchestrator.run()
console.log(report)