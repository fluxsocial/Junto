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

orchestrator.registerScenario('a test', async (s, t) => {
  const {agent1, agent2, agent3, agent4, agent5, agent6, agent7} = await s.players({agent1: mainConfig, agent2: mainConfig, agent3: mainConfig, agent4: mainConfig, agent5: mainConfig, agent6: mainConfig, agent7: mainConfig}, true);
  await scenarios.registerAgent(agent1, "jdeepee", "josh", "parkin")
  console.log("Completed agent1 registration\n\n\n")
  await scenarios.registerAgent(agent2, "sunyatax", "eric", "yang")
  console.log("Completed agent2 registration\n\n\n")
  await scenarios.registerAgent(agent3, "dora", "dora", "")
  console.log("Completed agent3 registration\n\n\n")
  await scenarios.registerAgent(agent4, "pog", "pogga", "dogga")
  console.log("Completed agent4 registration\n\n\n")
  await scenarios.registerAgent(agent5, "will", "will", "")
  console.log("Completed agent5 registration\n\n\n")
  await scenarios.registerAgent(agent6, "adam", "adam", "")
  console.log("Completed agent6 registration\n\n\n")
  await scenarios.registerAgent(agent7, "jessy", "jessy", "")
  console.log("Completed agent7 registration\n\n\n")
  await s.consistency()
  t.deepEqual(0, 0)
  // write some assertions
})

const report = orchestrator.run()
console.log(report)