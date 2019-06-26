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

    const post_2_expression = await scenarios.postExpression(t, agent1,
        {
            expression: {
                ShortForm: {
                    background: "",
                    body: "This is the second test expression"
                }
            },
            expression_type: "ShortForm"
        },
        ["social"],
        [holochain_env.Ok.dna_address]
    );

    const post_3_expression = await scenarios.postExpression(t, agent1,
        {
            expression: {
                ShortForm: {
                    background: "",
                    body: "This is the third test expression"
                }
            },
            expression_type: "ShortForm"
        },
        ["other"],
        [holochain_env.Ok.dna_address]
    );
    await s.consistent();
    const current_date = scenarios.getCurrentTimestamps();
    const can_query_1 = await scenarios.queryExpressions(t, agent1, "random",
                                                                    ["holochain<channel>", "Junto<channel>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                    "FilterNew",
                                                                    "ExpressionPost",
                                                                    "And",
                                                                    1,
                                                                    "otally random seed"); //0
    t.equal(can_query_1.Ok.length, 1);

    const can_query_1_and_2 = await scenarios.queryExpressions(t, agent1, "random",
                                                                    ["social<channel>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                    "FilterNew",
                                                                    "ExpressionPost",
                                                                    "And",
                                                                    1,
                                                                    "otally random seed"); //0
    t.equal(can_query_1_and_2.Ok.length, 2);

    can_query_3 = await scenarios.queryExpressions(t, agent1, "random",
                                                            ["other<channel>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                            "FilterNew",
                                                            "ExpressionPost",
                                                            "And",
                                                            1,
                                                            "otally random seed"); //0
    t.equal(can_query_3.Ok.length, 1);

    can_query_all = await scenarios.queryExpressions(t, agent1, "random",
                                                            [current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                            "FilterNew",
                                                            "ExpressionPost",
                                                            "And",
                                                            1,
                                                            "otally random seed"); //0
    t.equal(can_query_all.Ok.length, 3);

    can_get_1 = await scenarios.getExpression(t, agent1, post_1_expression.Ok);
});

diorama.run();