const { Config, Container, Scenario } = require("@holochain/holochain-nodejs");
//const n3h = require('n3h');
Scenario.setTape(require('tape'));

const dnaPath = "./dist/junto-rust.dna.json";

const dna = Config.dna(dnaPath);
const agentJosh = Config.agent("josh");
const instanceJosh = Config.instance(agentJosh, dna);
const scenario = new Scenario([instanceJosh]);

scenario.runTape('Retrieve den(s) and make auth operations on den.', async (t, {josh}) => {
    //create user
    const register_result = await josh.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}});
    console.log("Register user result", register_result);
    t.equal(JSON.stringify(register_result), JSON.stringify({ Ok: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn' }));
    console.log("Completed register profile\n\n\n");

    //get den(s)
    const get_den = await josh.callSync('core', 'user_dens', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'});
    console.log("Get den(s) result", get_den);
    t.equal(JSON.stringify(get_den), JSON.stringify({ Ok: 
        { private_den: '{"address":"QmV7H3Mhpdpj9NfFq2pgwzRd83uEjQupsHa5zwVVeCWSd2","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Private","channel_type":"Den"}}',
          shared_den: '{"address":"QmV9j9LNfc4spvT8qNA24vjMjC4JEnoVfidfiBfnY4PUs3","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Shared","channel_type":"Den"}}',
          public_den: '{"address":"Qmc48qWCdrCEqJVn1a4XZd6Eyrsu1W5jHHi1CgsVJgEAMx","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Public","channel_type":"Den"}}' } }));
    console.log("Completed get den results\n");

    //check current user is den owner
    const get_private_den_owner_status = await josh.callSync('core', 'is_den_owner', {den: 'QmV7H3Mhpdpj9NfFq2pgwzRd83uEjQupsHa5zwVVeCWSd2', user: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'});
    console.log("Get den(s) result", get_private_den_owner_status);
    t.equal(JSON.stringify(get_private_den_owner_status), JSON.stringify({ Ok: true }));

    const get_shared_den_owner_status = await josh.callSync('core', 'is_den_owner', {den: 'QmV9j9LNfc4spvT8qNA24vjMjC4JEnoVfidfiBfnY4PUs3', user: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'});
    console.log("Get den(s) result", get_shared_den_owner_status);
    t.equal(JSON.stringify(get_shared_den_owner_status), JSON.stringify({ Ok: true }));

    const get_public_den_owner_status = await josh.callSync('core', 'is_den_owner', {den: 'Qmc48qWCdrCEqJVn1a4XZd6Eyrsu1W5jHHi1CgsVJgEAMx', user: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'});
    console.log("Get den(s) result", get_public_den_owner_status);
    t.equal(JSON.stringify(get_public_den_owner_status), JSON.stringify({ Ok: true }));
    console.log("Completed get den owner status results\n");
});