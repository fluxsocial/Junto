const { Config, Container, Scenario } = require("@holochain/holochain-nodejs");
//const n3h = require('n3h');
Scenario.setTape(require('tape'));

const dnaPath = "./dist/junto-rust.dna.json";

const dna = Config.dna(dnaPath);
const agentJosh = Config.agent("josh");
const agentEric = Config.agent("eric");
const agentDora = Config.agent("dora");
const instanceJosh = Config.instance(agentJosh, dna);
const instanceEric = Config.instance(agentEric, dna);
const instanceDora = Config.instance(agentDora, dna);
const scenario = new Scenario([instanceJosh, instanceEric, instanceDora]);

scenario.runTape('Retrieve pack and make pack auth operations', async (t, {josh, eric, dora}) => {
        //create user josh
        const register_result_josh = await josh.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}});
        console.log("Register user josh result", register_result_josh);
        t.equal(JSON.stringify(register_result_josh), JSON.stringify({ Ok: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn' }));
        console.log("Completed register profile\n\n\n");

        //create user eric
        const register_result_eric = await eric.callSync('core', 'create_user', {user_data: {username: "sunyatax", first_name: "Eric", last_name: "Yang", bio: "Junto Testing", profile_picture: "pictureurl"}});
        console.log("Register user eric result", register_result_eric);
        t.equal(JSON.stringify(register_result_eric), JSON.stringify({ Ok: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU' }));
        console.log("Completed register profile\n\n\n");

        //create user dora
        const register_result_dora = await dora.callSync('core', 'create_user', {user_data: {username: "doracat", first_name: "Dora", last_name: "Liliom Czovek", bio: "Junto Testing", profile_picture: "pictureurl"}});
        console.log("Register user dora result", register_result_dora);
        t.equal(JSON.stringify(register_result_dora), JSON.stringify({ Ok: 'QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz' }));
        console.log("Completed register profile\n\n\n");

        //get joshs pack
        const get_josh_pack = await josh.callSync('core', 'get_user_pack', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'});
        console.log("Get josh pack result", get_josh_pack);
        t.equal(JSON.stringify(get_josh_pack), JSON.stringify({ Ok: '{"address": "QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA", "entry": {"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}}' }));
        console.log("Completed get josh pack\n\n\n");

        //get erics pack
        const get_eric_pack = await eric.callSync('core', 'get_user_pack', {username_address: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'});
        console.log("Get eric pack result", get_eric_pack);
        t.equal(JSON.stringify(get_eric_pack), JSON.stringify({ Ok: '{"address": "QmTevRrtjaaJzNCESubqfZfZjNXxJH4RxFBKFb9Nd7LUWh", "entry": {"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Pack","owner":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","privacy":"Shared"}}' }));
        console.log("Completed get eric pack\n\n\n");

        const add_pack_member = await josh.callSync('core', 'add_to_pack', {username_address: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'}); //add eric to josh's pack
        t.equal(JSON.stringify(add_pack_member), JSON.stringify({ Ok: { message: 'User added to pack' } }));
        console.log("add pack member result", add_pack_member);
        console.log("Completed add pack member to josh's pack\n\n\n");

        const get_pack_members_by_owner = await josh.callSync('core', 'get_pack_members', {pack: 'QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA'});
        t.equal(JSON.stringify(get_pack_members_by_owner), JSON.stringify({ Ok: '{"members":[{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}}]}' }));
        console.log("get pack member by owner", get_pack_members_by_owner);
        console.log("Completed get pack members by owner\n\n\n");

        const get_pack_members_by_member = await eric.callSync('core', 'get_pack_members', {pack: 'QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA'});
        t.equal(JSON.stringify(get_pack_members_by_member), JSON.stringify({ Ok: '{"members":[{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}}]}' }));
        console.log("get pack member by owner", get_pack_members_by_member);
        console.log("Completed get pack members by owner\n\n\n");

        const get_user_member_non_member_or_owner = await dora.callSync('core', 'get_pack_members', {pack: 'QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA'});
        t.equal(JSON.stringify(get_user_member_non_member_or_owner), JSON.stringify({ Err: { Internal: 'You are not an owner or member of this pack and thus are not allowed to view given information' } }));
        console.log("get pack member by owner", get_user_member_non_member_or_owner);
        console.log("Completed get pack members by owner\n\n\n");

        const is_pack_member = await dora.callSync('core', 'is_pack_member', {pack: 'QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA', user: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'});
        t.equal(JSON.stringify(is_pack_member), JSON.stringify({Ok: true}));
        console.log("is pack member result", is_pack_member);
        console.log("Completed is pack member\n\n\n");

        const is_not_pack_member = await dora.callSync('core', 'is_pack_member', {pack: 'QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA', user: 'QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz'});
        t.equal(JSON.stringify(is_not_pack_member), JSON.stringify({Ok: false}));
        console.log("is not pack member result", is_not_pack_member);
        console.log("Completed is not pack member\n\n\n");
});