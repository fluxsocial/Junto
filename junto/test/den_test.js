const {Diorama, tapeExecutor} = require('@holochain/diorama')
const scenarios = require("./scenarios.js")
const dnaPath = Diorama.dna('./dist/junto-rust.dna.json', 'junto')

const diorama = new Diorama({
    instances: {
      agent1: dnaPath
    },
    debugLog: false,
    executor: tapeExecutor(require('tape'))
});

diorama.registerScenario('Retrieve den(s) and make auth operations on den', async (s, t, {agent1}) => {
    const user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin");
    await s.consistent();
    const get_dens = await scenarios.getDens(t, agent1, user1.Ok.username.address);
    t.equal(JSON.stringify(get_dens), JSON.stringify({"Ok":{"private_den":{"address":"QmNM4SrnDweAjAwNUrBoSkbpEW8G4YUHg5jihoY5VYbsoG","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"joshua\'s Den","privacy":"Private"}},"shared_den":{"address":"QmdmsxPHxWKBDn3hiaAU1mNCr58SX8r3p5PVAcrkJJwQgJ","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"joshua\'s Den","privacy":"Shared"}},"public_den":{"address":"QmfYPrDCa53A7bJBQvAFF84S3755g8fuhy1rmQVnBFWghi","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"joshua\'s Den","privacy":"Public"}}}}));

    const private_den_owner = await scenarios.isCollectionOwner(t, agent1, get_dens.Ok.private_den.address, user1.Ok.username.address);
    t.equal(JSON.stringify(private_den_owner), JSON.stringify({ Ok: true }));

    const shared_den_owner = await scenarios.isCollectionOwner(t, agent1, get_dens.Ok.shared_den.address, user1.Ok.username.address);
    t.equal(JSON.stringify(shared_den_owner), JSON.stringify({ Ok: true }));

    const public_den_owner = await scenarios.isCollectionOwner(t, agent1, get_dens.Ok.public_den.address, user1.Ok.username.address);
    t.equal(JSON.stringify(private_den_owner), JSON.stringify({ Ok: true }));
});

diorama.run();