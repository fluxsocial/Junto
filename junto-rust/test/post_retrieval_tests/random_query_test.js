const {Diorama, tapeExecutor} = require('@holochain/diorama')
const scenarios = require("../scenarios.js")
const dnaPath = Diorama.dna('./dist/junto-rust.dna.json', 'junto')

const diorama = new Diorama({
    instances: {
      agent1: dnaPath
    },
    debugLog: false,
    executor: tapeExecutor(require('tape'))
});

diorama.registerScenario('Can post expression and do basic random query', async (s, t, {agent1}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 1);
    await s.consistent();

    const post_global_expression = await scenarios.postExpression(t, agent1,
        {
            expression: {
                ShortForm: {
                    background: "",
                    body: "This is the first test expression"
                }
            },
            expression_type: "ShortForm"
        },
        ["holochain", "Junto", "social", "holo"],
        [holochain_env.Ok.dna_address]
    );
    await s.consistent();
    const current_date = scenarios.getCurrentTimestamps();
    const random_query = await scenarios.getExpression(t, agent1, "random",
                                                                    ["social<channel>", "junto<channel>", "holochain<channel>", "holo<channel>", "jdeepee<user>", "shortform<type>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                    "FilterNew",
                                                                    "ExpressionPost",
                                                                    "And",
                                                                    1,
                                                                    "otally random seed"); //0
    t.equal(JSON.stringify(random_query), JSON.stringify({"Ok":[{"address":"QmT9LnUxYb6dBUpwvwfDnLTsDcKTAmKYqj9LHcW3ZWyyQW","entry":{"expression_type":"ShortForm","expression":{"ShortForm":{"background":"","body":"This is the first test expression"}}}}]}));
});

diorama.run();