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
    const post_1_comment = await scenarios.postComment(t, agent1,
        {
            expression: {
                ShortForm: {
                    background: "",
                    body: "This is the first test comment expression"
                }
            },
            expression_type: "ShortForm"
        },
        post_1_expression.Ok
    );

    const can_get_comment = await scenarios.getExpression(t, agent1, post_1_expression.Ok);
    t.equal(can_get_comment.Ok.sub_expressions.length, 1);
    t.equal(can_get_comment.Ok.sub_expressions_count, 1)
});

diorama.run();