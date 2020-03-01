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

orchestrator.registerScenario('Simple perspective query test', async (s, t) => {
    const {agent1, agent2} = await s.players({agent1: mainConfig, agent2: mainConfig}, true);
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    const user2 = await scenarios.registerAgent(t, agent2, "sunyatax", "eric", "yang");
    await s.consistency();
    const add_user = await scenarios.addUserToPerspective(t, agent1, user1.Ok.user_perspective.address, user2.Ok.username.address);
    const holochain_env = await scenarios.getHolochainEnv(t, agent1);
    const update_bit_prefix = await scenarios.updateBitPrefix(t, agent1, 2);
    await s.consistency();
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
    await s.consistency();
    const current_date = scenarios.getCurrentTimestamps();
    let current_month = (current_date.month < 10) ? "0"+ current_date.month : current_date.month;
    let current_year = (current_date.year < 10) ? "0"+ current_date.year : current_date.year;
    let current_day = (current_date.day < 10) ? "0" + current_date.day : current_date.day;
    const perspective_query = await scenarios.queryExpressions(t, agent1, user1.Ok.user_perspective.address,
        ["social<channel>", "junto<channel>", "holochain<channel>", "holo<channel>", "sunyatax<user>", "shortform<type>", current_year+"<time:y>", current_month+"<time:m>", current_day+"<time:d>", current_date.hour+"<time:h>"],
        "FilterNew",
        "ExpressionPost",
        "And",
        1,
        "totally random seed",
        false);
    t.equal(perspective_query.Ok.length, 1)
});

const report = orchestrator.run()
console.log(report);