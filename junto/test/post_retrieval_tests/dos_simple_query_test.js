const { Orchestrator, Config } = require('@holochain/tryorama');
const scenarios = require("../scenarios.js");

const dnaJunto = Config.dna('./dist/junto.dna.json', 'junto');

const mainConfig = Config.gen(
    {
      junto: dnaJunto,  // agent_id="blog", instance_id="blog", dna=dnaBlog
    },
    {
        // specify a bridges
        bridges: [],
        logger: {
            type: 'debug',
            state_dump: false,
            rules: {
                rules: [{ exclude: true, pattern: ".*" }]
            }
        },
        // use a sim2h network
        network: {
            type: 'sim2h',
            sim2h_url: 'wss://sim2h.holochain.org:9000',
        },
    }
);

String.prototype.format = function() {
    var formatted = this;
    for (var i = 0; i < arguments.length; i++) {
        var regexp = new RegExp('\\{'+i+'\\}', 'gi');
        formatted = formatted.replace(regexp, arguments[i]);
    }
    return formatted;
};

const orchestrator = new Orchestrator();

orchestrator.registerScenario('Simple DOS query tes', async (s, t) => {
    const {user1, user2} = await s.players({user1: mainConfig, user2: mainConfig}, true);
    const user1_res = await scenarios.registerAgent(t, user1, "jdeepee", "joshua", "parkin");
    const agent_private_den = user1_res.Ok.private_den.address;
    const user2_res = await scenarios.registerAgent(t, user2, "sunyatax", "eric", "yang");
    await s.consistency();

    const add_pack_member = await scenarios.addPackMember(t, user2, "QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn");
    const holochain_env = await scenarios.getHolochainEnv(t, user1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, user1, 2);
    const post_global_expression = await scenarios.postExpression(t, user1,
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
    await s.consistency();
    const post_private_expression = await scenarios.postExpression(t, user1,
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
    await s.consistency();                                                                    
    const current_date = scenarios.getCurrentTimestamps();
    const make_1_dos_query = await scenarios.queryExpressions(t, user2, "dos", 
                                                                        ["social<channel>", "junto<channel>", "holochain<channel>", "holo<channel>", "jdeepee<user>", "shortform<type>", current_date.year+"<time:y>", current_date.month+"<time:m>", current_date.day+"<time:d>", current_date.hour+"<time:h>"],
                                                                        "FilterNew",
                                                                        "ExpressionPost",
                                                                        "And",
                                                                        1,
                                                                        "totally random seed",
                                                                        false);
    t.equal(make_1_dos_query.Ok.length, 1);
});

const report = orchestrator.run()
console.log(report)
