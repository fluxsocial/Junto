const {Diorama, tapeExecutor} = require('@holochain/diorama')
const scenarios = require("./scenarios.js")
const dnaPath = Diorama.dna('./dist/junto-rust.dna.json', 'junto')

const diorama = new Diorama({
    instances: {
      agent1: dnaPath,
      agent2: dnaPath,
      agent3: dnaPath
    },
    debugLog: false,
    executor: tapeExecutor(require('tape'))
});

diorama.registerScenario('Retrieve pack and make pack auth operations', async (s, t, {agent1, agent2, agent3}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    const user3 = await scenarios.registerAgent(t, agent3, "doracat", "dora", "Liliom Czovek");
    await s.consistent();

    const agent1_pack = await scenarios.getUserPack(t, agent1, user1.Ok.username.address);
    const agent2_pack = await scenarios.getUserPack(t, agent2, user2.Ok.username.address);

    const add_pack_member = await scenarios.addPackMember(t, agent1, user2.Ok.username.address);
    const get_group_members_by_owner = await scenarios.getGroupMembers(t, agent1, user1.Ok.pack.address, true);
    const get_group_members_by_member = await scenarios.getGroupMembers(t, agent2, user1.Ok.pack.address, true);
    const get_group_member_by_non_owner_or_member = await scenarios.getGroupMembers(t, agent3, user1.Ok.pack.address, false);

    const is_group_member = await scenarios.isGroupMember(t, agent3, user1.Ok.pack.address, user2.Ok.username.address);
    t.equal(JSON.stringify(is_group_member), JSON.stringify({Ok: true}));
    const is_not_group_member = await scenarios.isGroupMember(t, agent3, user1.Ok.pack.address, user3.Ok.username.address);
    t.equal(JSON.stringify(is_not_group_member), JSON.stringify({Ok: false}));

    const remove_group_member = await scenarios.removeGroupMember(t, agent1, user2.Ok.username.address, user1.Ok.pack.address);
    const check_removed = await scenarios.getGroupMembers(t, agent1, user1.Ok.pack.address, true);
    t.equal(JSON.stringify(check_removed), JSON.stringify({ Ok: '{"members":[]}' }));
});

diorama.run();