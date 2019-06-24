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

diorama.registerScenario('Simple perspective query test', async (s, t, {agent1, agent2}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    await s.consistent();
    const add_user = await scenarios.addUserToPerspective(t, agent1, user1.Ok.user_perspective.address, user2.Ok.username.address);
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 2);
    await s.consistent();
    const post_global_expression = await scenarios.postExpression(t, agent2,
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
    const perspective_query = await scenarios.getExpression(t, agent1, user1.Ok.user_perspective.address,
                                                                        ["social<channel>", "junto<channel>", "holochain<channel>", "holo<channel>", "sunyatax<user>", "shortform<type>", current_date.year+"<time:y>", "0"+current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                        "FilterNew",
                                                                        "ExpressionPost",
                                                                        "And",
                                                                        1,
                                                                        "totally random seed");
    t.equal(JSON.stringify(perspective_query), JSON.stringify({"Ok":[{"expression":{"address":"QmT9LnUxYb6dBUpwvwfDnLTsDcKTAmKYqj9LHcW3ZWyyQW","entry":{"expression_type":"ShortForm","expression":{"ShortForm":{"background":"","body":"This is the first test expression"}}}},"sub_expressions":[],"author_username":{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}},"author_profile":{"address":"Qmaao8yPQtLA7Muo8xxJFCvYFKf7m1HbNzrhtN9JUPHeiv","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","first_name":"eric","last_name":"yang","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"resonations":[],"timestamp":"{0}-0{1}-{2}-{3}".format(current_date.year, current_date.month, current_date.day, current_date.hour),"channels":[{"address":"QmdPBmDreYonmoAvTqbYxJxaT3ieb82cEmZw6WQdhUUgPe","entry":{"value":"social","attribute_type":"Channel"}},{"address":"QmcwZceeJ5nTzetNG9CKA493fPEnnrs3A8JUpGvt5B7CfG","entry":{"value":"junto","attribute_type":"Channel"}},{"address":"QmWkARhLBLzCgr1vgf8fh9597kr23QsgJSP3tMNTX2DyRm","entry":{"value":"holochain","attribute_type":"Channel"}},{"address":"QmU5oKkpaqEZK1J6Fc9Fjrk3tT8929JfJKB65eFe65HeDf","entry":{"value":"holo","attribute_type":"Channel"}}]}]}));
});

diorama.run();