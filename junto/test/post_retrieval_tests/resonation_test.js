const {Diorama, tapeExecutor} = require('@holochain/diorama')
const scenarios = require("../scenarios.js")
const dnaPath = Diorama.dna('./dist/junto-rust.dna.json', 'junto')

const diorama = new Diorama({
    instances: {
      agent1: dnaPath,
      agent2: dnaPath
    },
    debugLog: false,
    executor: tapeExecutor(require('tape'))
});

diorama.registerScenario('Can make and retrieve resonation', async (s, t, {agent1, agent2}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 1);
    await s.consistent();

    const post_1_expression = await scenarios.postExpression(t, agent1,
        {
            expression: {
                ShortForm: {
                    background: "",
                    body: "This is the first test expression"
                }
            },
            expression_type: "ShortForm"
        },
        ["holochain", "Junto", "social"],
        [holochain_env.Ok.dna_address]
    );
    const current_date = scenarios.getCurrentTimestamps();
    const make_resonation = await scenarios.resonation(t, agent2, post_1_expression.Ok);
    await s.consistent();

    const get_expression_with_resonation = await scenarios.getExpression(t, agent2, post_1_expression.Ok);
    console.log("Get expression");
    t.equal(get_expression_with_resonation.Ok.resonations.length, 1);

    const get_resonation_from_group = await scenarios.queryExpressions(t, agent2, user2.Ok.pack.address,
                                                                                    ["social<channel>", "junto<channel>", "holochain<channel>", "jdeepee<user>", "shortform<type>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                                    "FilterNew",
                                                                                    "ExpressionPost",
                                                                                    "And",
                                                                                    1,
                                                                                    "otally random seed",
                                                                                    true);
    console.log("Resonation get");
    t.equal(get_resonation_from_group.Ok.length, 1);
});

diorama.run()