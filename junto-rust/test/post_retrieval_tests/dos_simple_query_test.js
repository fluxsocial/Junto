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
                                                                        "totally random seed");
    t.equal(JSON.stringify(make_1_dos_query), JSON.stringify({"Ok":[{"expression":{"address":"QmT9LnUxYb6dBUpwvwfDnLTsDcKTAmKYqj9LHcW3ZWyyQW","entry":{"expression_type":"ShortForm","expression":{"ShortForm":{"background":"","body":"This is the first test expression"}}}},"sub_expressions":[],"author_username":{"address":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","entry":{"username":"jdeepee"}},"author_profile":{"address":"QmVEh39ZzEYG3B1T6FJ1zpMCwRV5WsKjg1ek5zocvD9Tnt","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"joshua","last_name":"parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"resonations":[],"timestamp":"{0}-0{1}-{2}-{3}".format(current_date.year, current_date.month, current_date.day, current_date.hour),"channels":[{"address":"QmdPBmDreYonmoAvTqbYxJxaT3ieb82cEmZw6WQdhUUgPe","entry":{"value":"social","attribute_type":"Channel"}},{"address":"QmcwZceeJ5nTzetNG9CKA493fPEnnrs3A8JUpGvt5B7CfG","entry":{"value":"junto","attribute_type":"Channel"}},{"address":"QmWkARhLBLzCgr1vgf8fh9597kr23QsgJSP3tMNTX2DyRm","entry":{"value":"holochain","attribute_type":"Channel"}},{"address":"QmU5oKkpaqEZK1J6Fc9Fjrk3tT8929JfJKB65eFe65HeDf","entry":{"value":"holo","attribute_type":"Channel"}}]}]}));
});

diorama.run();