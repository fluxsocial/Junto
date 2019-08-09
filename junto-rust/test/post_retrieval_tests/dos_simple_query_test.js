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

String.prototype.format = function() {
    var formatted = this;
    for (var i = 0; i < arguments.length; i++) {
        var regexp = new RegExp('\\{'+i+'\\}', 'gi');
        formatted = formatted.replace(regexp, arguments[i]);
    }
    return formatted;
};

diorama.registerScenario('Simple DOS query tes', async (s, t, {agent1, agent2}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const agent_private_den = user1.Ok.private_den.address;
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    await s.consistent();
    const add_pack_member = await scenarios.addPackMember(t, agent2, "QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn");
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 2);
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
    const post_private_expression = await scenarios.postExpression(t, agent1,
                                                                        {
                                                                            expression: {
                                                                                ShortForm: {
                                                                                    background: "",
                                                                                    body: "This is the second test expression"
                                                                                }
                                                                            },
                                                                            expression_type: "ShortForm"
                                                                        },
                                                                        ["holochain", "Junto", "social", "holo"],
                                                                        [agent_private_den]
                                                                    );
    await s.consistent();                                                                    
    const current_date = scenarios.getCurrentTimestamps();
    const make_1_dos_query = await scenarios.queryExpressions(t, agent2, "dos", 
                                                                        ["social<channel>", "junto<channel>", "holochain<channel>", "holo<channel>", "jdeepee<user>", "shortform<type>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                        "FilterNew",
                                                                        "ExpressionPost",
                                                                        "And",
                                                                        1,
                                                                        "totally random seed",
                                                                        false);
    t.equal(make_1_dos_query.Ok.length, 1);
});

diorama.run();