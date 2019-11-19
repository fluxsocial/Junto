const {Diorama, tapeExecutor} = require('@holochain/diorama')
const scenarios = require("./scenarios.js")
const dnaPath = Diorama.dna('./dist/junto-rust.dna.json', 'junto')

const diorama = new Diorama({
    instances: {
        agent1: dnaPath,
        agent2: dnaPath
    },
    debugLog: false,
    executor: tapeExecutor(require('tape'))
});

diorama.registerScenario('Can add and get users from perspective', async (s, t, {agent1, agent2}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    await s.consistent();

    const add_user = await scenarios.addUserToPerspective(t, agent1, user1.Ok.user_perspective.address, user2.Ok.username.address);
    const get_users = await scenarios.getPerspectivesUsers(t, agent1, user1.Ok.user_perspective.address);
    t.equal(JSON.stringify(get_users), JSON.stringify({"Ok":[user2.Ok.username]}));
});

diorama.run();