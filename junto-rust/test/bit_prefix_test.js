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

diorama.registerScenario('Can add, get users from perspective and get posts from a perspective', async (s, t, {agent1}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 2);
});

diorama.run();