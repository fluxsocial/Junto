// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');

const { Config, Container } = require("@holochain/holochain-nodejs")

const dnaPath = "./dist/bundle.json"

// closure to keep config-only stuff out of test scope
const container = (() => {
    const agentJosh = Config.agent("josh")

    const dna = Config.dna(dnaPath)

    const instanceJosh = Config.instance(agentJosh, dna)

    const containerConfig = Config.container([instanceJosh])
    return new Container(containerConfig)
})()

// Initialize the Container
container.start()

const app = container.makeCaller('josh', dnaPath)

test('Creating user test', (t) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
    let user = app.call("core", "main", "create_user", {user_data : {
      parent: "hashstring", //Parent HashString data objects to be contextual to given data trees
      first_name: "Test user",
      last_name: "Test Last",
      bio: "Bio test user",
      profile_picture: "picture test user",
      verified: true}})

  // check for equality of the actual and expected results
  t.deepEqual(user, { Ok: { App: [ 'my_entry', '{"content":"sample content"}' ] } })

  // ends this test
  t.end()
})
