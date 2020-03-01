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

const orchestrator = new Orchestrator();

String.prototype.format = function() {
    var formatted = this;
    for (var i = 0; i < arguments.length; i++) {
        var regexp = new RegExp('\\{'+i+'\\}', 'gi');
        formatted = formatted.replace(regexp, arguments[i]);
    }
    return formatted;
};

orchestrator.registerScenario('Can post expression and do basic random query', async (s, t) => {
    const {agent1} = await s.players({agent1: mainConfig}, true);
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 1);
    await s.consistency();

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
    await s.consistency();
    const current_date = scenarios.getCurrentTimestamps();
    let current_month = (current_date.month < 10) ? "0"+ current_date.month : current_date.month;
    let current_year = (current_date.year < 10) ? "0"+ current_date.year : current_date.year;
    let current_day = (current_date.day < 10) ? "0" + current_date.day : current_date.day;
    const can_query_1 = await scenarios.queryExpressions(t, agent1, "random",
                                                                    ["holochain<channel>", "Junto<channel>", current_year+"<time:y>", current_month+"<time:m>", current_day+"<time:d>", current_date.hour+"<time:h>"],
                                                                    "FilterNew",
                                                                    "ExpressionPost",
                                                                    "And",
                                                                    1,
                                                                    "otally random seed",
                                                                    false); //0
    t.equal(can_query_1.Ok.length, 1);

    const can_query_1_and_2 = await scenarios.queryExpressions(t, agent1, "random",
                                                                    ["social<channel>", current_year+"<time:y>", current_month+"<time:m>", current_day+"<time:d>", current_date.hour+"<time:h>"],
                                                                    "FilterNew",
                                                                    "ExpressionPost",
                                                                    "And",
                                                                    1,
                                                                    "otally random seed",
                                                                    false); //0
    t.equal(can_query_1_and_2.Ok.length, 2);

    can_query_3 = await scenarios.queryExpressions(t, agent1, "random",
                                                            ["other<channel>", current_year+"<time:y>", current_month+"<time:m>", current_day+"<time:d>", current_date.hour+"<time:h>"],
                                                            "FilterNew",
                                                            "ExpressionPost",
                                                            "And",
                                                            1,
                                                            "otally random seed",
                                                            false); //0
    t.equal(can_query_3.Ok.length, 1);

    can_query_all = await scenarios.queryExpressions(t, agent1, "random",
                                                            [current_year+"<time:y>", current_month+"<time:m>", current_day+"<time:d>", current_date.hour+"<time:h>"],
                                                            "FilterNew",
                                                            "ExpressionPost",
                                                            "And",
                                                            1,
                                                            "otally random seed",
                                                            false); //0
    t.equal(can_query_all.Ok.length, 3);

    can_get_1 = await scenarios.getExpression(t, agent1, post_1_expression.Ok);
});

const report = orchestrator.run()
console.log(report);