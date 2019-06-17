const {Diorama, tapeExecutor} = require('@holochain/diorama')
const scenarios = require("../scenarios.js")
const juntoDna = Diorama.dna('./dist/junto-rust.dna.json', 'junto')

const diorama = new Diorama({
    instances: {
      agent1: juntoDna,
      agent2: juntoDna,
      agent3: juntoDna,
      agent4: juntoDna,
      agent5: juntoDna,
      agent6: juntoDna,
      agent7: juntoDna
    },
    debugLog: false,
    executor: tapeExecutor(require('tape'))
  })

  diorama.registerScenario('a test', async (s, t, {agent1, agent2, agent3, agent4, agent5, agent6, agent7}) => {
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
    await s.consistent()
    t.deepEqual(0, 0)
    // write some assertions
})

diorama.run()