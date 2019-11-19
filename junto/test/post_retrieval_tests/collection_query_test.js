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

String.prototype.format = function() {
    var formatted = this;
    for (var i = 0; i < arguments.length; i++) {
        var regexp = new RegExp('\\{'+i+'\\}', 'gi');
        formatted = formatted.replace(regexp, arguments[i]);
    }
    return formatted;
};

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
    const random_query = await scenarios.queryExpressions(t, agent1, user1.Ok.private_den.address,
                                                                    ["social<channel>", "junto<channel>", "holochain<channel>", "holo<channel>", "jdeepee<user>", "shortform<type>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                    "FilterNew",
                                                                    "ExpressionPost",
                                                                    "And",
                                                                    1,
                                                                    "otally random seed",
                                                                    false); //0
    t.equal(random_query.Ok.length, 1);
});

diorama.run();