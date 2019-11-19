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

orchestrator.registerScenario('Retrieve pack and make pack auth operations', async (s, t) => {
    const {agent1, agent2, agent3} = await s.players({agent1: mainConfig, agent2: mainConfig, agent3: mainConfig}, true);
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    const user3 = await scenarios.registerAgent(t, agent3, "doracat", "dora", "Liliom Czovek");
    await s.consistency();

    const agent1_pack = await scenarios.getUserPack(t, agent1, user1.Ok.username.address);
    const agent2_pack = await scenarios.getUserPack(t, agent2, user2.Ok.username.address);
    await s.consistency();

    const add_pack_member = await scenarios.addPackMember(t, agent1, user2.Ok.username.address);
    await s.consistency();
    const get_group_members_by_owner = await scenarios.getGroupMembers(t, agent1, user1.Ok.pack.address, true);
    const get_group_members_by_member = await scenarios.getGroupMembers(t, agent2, user1.Ok.pack.address, true);
    const get_group_member_by_non_owner_or_member = await scenarios.getGroupMembers(t, agent3, user1.Ok.pack.address, false);

    const is_group_member = await scenarios.isGroupMember(t, agent3, user1.Ok.pack.address, user2.Ok.username.address);
    t.equal(JSON.stringify(is_group_member), JSON.stringify({Ok: true}));
    const is_not_group_member = await scenarios.isGroupMember(t, agent3, user1.Ok.pack.address, user3.Ok.username.address);
    t.equal(JSON.stringify(is_not_group_member), JSON.stringify({Ok: false}));

    const remove_group_member = await scenarios.removeGroupMember(t, agent1, user2.Ok.username.address, user1.Ok.pack.address);
    await s.consistency();
    const check_removed = await scenarios.getGroupMembers(t, agent1, user1.Ok.pack.address, true);
    t.equal(JSON.stringify(check_removed), JSON.stringify({ Ok: '{"members":[]}' }));
});

const report = orchestrator.run()
console.log(report)
