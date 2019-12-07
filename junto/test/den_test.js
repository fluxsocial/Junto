const { Orchestrator, Config } = require('@holochain/tryorama');
const scenarios = require("./scenarios.js");

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

orchestrator.registerScenario('Retriev den(s) and make auth  operations on den', async (s, t) => {
    const {agent1} = await s.players({agent1: mainConfig}, true);
    let user1 = await scenarios.registerAgent(t, agent1, "jdeepee", "joshua", "parkin").catch(err => { console.log(err) } );
    await s.consistency();

    const get_dens = await scenarios.getDens(t, agent1, user1.Ok.username.address);
    t.equal(JSON.stringify(get_dens), JSON.stringify({"Ok":{"private_den":{"address":"QmNM4SrnDweAjAwNUrBoSkbpEW8G4YUHg5jihoY5VYbsoG","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"joshua\'s Den","privacy":"Private"}},"shared_den":{"address":"QmdmsxPHxWKBDn3hiaAU1mNCr58SX8r3p5PVAcrkJJwQgJ","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"joshua\'s Den","privacy":"Shared"}},"public_den":{"address":"QmfYPrDCa53A7bJBQvAFF84S3755g8fuhy1rmQVnBFWghi","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"joshua\'s Den","privacy":"Public"}}}}));

    const private_den_owner = await scenarios.isCollectionOwner(t, agent1, get_dens.Ok.private_den.address, user1.Ok.username.address);
    t.equal(JSON.stringify(private_den_owner), JSON.stringify({ Ok: true }));

    const shared_den_owner = await scenarios.isCollectionOwner(t, agent1, get_dens.Ok.shared_den.address, user1.Ok.username.address);
    t.equal(JSON.stringify(shared_den_owner), JSON.stringify({ Ok: true }));

    const public_den_owner = await scenarios.isCollectionOwner(t, agent1, get_dens.Ok.public_den.address, user1.Ok.username.address);
    t.equal(JSON.stringify(private_den_owner), JSON.stringify({ Ok: true }));
});

const report = orchestrator.run()
console.log(report)