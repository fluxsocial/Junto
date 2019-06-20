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
        t.equal(JSON.stringify(register_result_josh), JSON.stringify({"Ok":{"private_den":{"address":"QmV7H3Mhpdpj9NfFq2pgwzRd83uEjQupsHa5zwVVeCWSd2","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmV9j9LNfc4spvT8qNA24vjMjC4JEnoVfidfiBfnY4PUs3","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"Qmc48qWCdrCEqJVn1a4XZd6Eyrsu1W5jHHi1CgsVJgEAMx","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC","entry":{"name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}},"profile":{"address":"QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","entry":{"username":"jdeepee"}},"user_perspective":{"address":"QmcBgVN5mo8ACrX1Z1f2ZXNFzbRWSGhMskuNoJXe9fYQ71","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
        console.log("Completed register profile\n\n\n");

        //create user eric
        const register_result_eric = await eric.callSync('core', 'create_user', {user_data: {username: "sunyatax", first_name: "Eric", last_name: "Yang", bio: "Junto Testing", profile_picture: "pictureurl"}});
        console.log("Register user eric result", register_result_eric);
        t.equal(JSON.stringify(register_result_eric), JSON.stringify({"Ok":{"private_den":{"address":"QmXi3gko95vvsYWhdTocjBPHTUZsBKnu9coZc4EJKMkwFe","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmTunjPizi21fTRU23WufNVMFAXLd1XRnYDab9nynrYgKD","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmUWkBB8ttdEUf6nEyMWodrJQcgv9fuQKFuRg7A6hKSj41","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmdZyapF7huQvjsB8tUw7riBXU1hoDDVYdG94qCgbvKpw8","entry":{"name":"Eric\'s Pack","owner":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","privacy":"Shared"}},"profile":{"address":"QmXF2BASNKjFg76hth4b6PJ4Btj6oeJLmn7AuZerCLrZiM","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","first_name":"Eric","last_name":"Yang","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}},"user_perspective":{"address":"QmSxHAHwnE3Qw76uS6jviKMkVW1g3LCU828hxsqKqU6dpd","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
        console.log("Completed register profile\n\n\n");

        //create user dora
        const register_result_dora = await dora.callSync('core', 'create_user', {user_data: {username: "doracat", first_name: "Dora", last_name: "Liliom Czovek", bio: "Junto Testing", profile_picture: "pictureurl"}});
        console.log("Register user dora result", register_result_dora);
        t.equal(JSON.stringify(register_result_dora), JSON.stringify({"Ok":{"private_den":{"address":"QmW6e5Dj9ALVxD9jZce6dfx1WtgosyeSczGfgSf6GPDPZr","entry":{"parent":"QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz","name":"Dora\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmWWdGfwqhNCtFx9fz2BiiMNoGA7adnnsUXhggRWzc1kNw","entry":{"parent":"QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz","name":"Dora\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmbSP2RtMqmRKAG1LBv3RxSMoQ9faRAV8nc7egukD3sEME","entry":{"parent":"QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz","name":"Dora\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmQNkoBNJBLvzkNK5xVGjEQrqgiBmB8UvcByRutvrWYU3B","entry":{"name":"Dora\'s Pack","owner":"QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz","privacy":"Shared"}},"profile":{"address":"QmXbgC8Z4YrSbBeirjrShhTTK9zPmUmRUNeav98mHuDa6n","entry":{"parent":"QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz","first_name":"Dora","last_name":"Liliom Czovek","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz","entry":{"username":"doracat"}},"user_perspective":{"address":"QmS9393bnQrZt3phLdbeCeN873MjDpEsB711Kzu4rrjyj4","entry":{"parent":"QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
        console.log("Completed register profile\n\n\n");

        //get joshs pack
        const get_josh_pack = await josh.callSync('core', 'user_pack', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'});
        console.log("Get josh pack result", get_josh_pack);
        t.equal(JSON.stringify(get_josh_pack), JSON.stringify({"Ok":{"address":"QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC","entry":{"name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}}}));
        console.log("Completed get josh pack\n\n\n");

        //get erics pack
        const get_eric_pack = await eric.callSync('core', 'user_pack', {username_address: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'});
        console.log("Get eric pack result", get_eric_pack);
        t.equal(JSON.stringify(get_eric_pack), JSON.stringify({"Ok":{"address":"QmdZyapF7huQvjsB8tUw7riBXU1hoDDVYdG94qCgbvKpw8","entry":{"name":"Eric\'s Pack","owner":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","privacy":"Shared"}}}));
        console.log("Completed get eric pack\n\n\n");

        const add_group_member = await josh.callSync('core', 'add_pack_member', {username_address: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'}); //add eric to josh's group
        t.equal(JSON.stringify(add_group_member), JSON.stringify({ Ok: { message: 'User added to group' } }));
        console.log("add group member result", add_group_member);
        console.log("Completed add group member to josh's group\n\n\n");

        const get_group_members_by_owner = await josh.callSync('core', 'group_members', {group: 'QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC'});
        t.equal(JSON.stringify(get_group_members_by_owner), JSON.stringify({ Ok: '{"members":[{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}}]}' }));
        console.log("get group member by owner", get_group_members_by_owner);
        console.log("Completed get group members by owner\n\n\n");

        const get_group_members_by_member = await eric.callSync('core', 'group_members', {group: 'QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC'});
        t.equal(JSON.stringify(get_group_members_by_member), JSON.stringify({ Ok: '{"members":[{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}}]}' }));
        console.log("get group member by owner", get_group_members_by_member);
        console.log("Completed get group members by owner\n\n\n");

        const get_user_member_non_member_or_owner = await dora.callSync('core', 'group_members', {group: 'QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC'});
        t.equal(JSON.stringify(get_user_member_non_member_or_owner), JSON.stringify({ Err: { Internal: 'You are not an owner or member of this group and thus are not allowed to view given information' } }));
        console.log("get group member by non member or owner", get_user_member_non_member_or_owner);
        console.log("Completed get group members by non member or owner\n\n\n");

        const is_group_member = await dora.callSync('core', 'is_group_member', {group: 'QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC', user: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'});
        t.equal(JSON.stringify(is_group_member), JSON.stringify({Ok: true}));
        console.log("is group member result", is_group_member);
        console.log("Completed is group member\n\n\n");

        const is_not_group_member = await dora.callSync('core', 'is_group_member', {group: 'QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC', user: 'QmWAJausHsWxGvpJJrwtiu6nyQyXDMX426NdghjmHrVfSz'});
        t.equal(JSON.stringify(is_not_group_member), JSON.stringify({Ok: false}));
        console.log("is not group member result", is_not_group_member);
        console.log("Completed is not group member\n\n\n");

        const remove_group_member = await josh.callSync('core', 'remove_group_member', {username_address: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU', group: 'QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC'});
        t.equal(JSON.stringify(remove_group_member), JSON.stringify({ Ok: { "message": "User removed from group" }}));
        console.log("remove group member result", remove_group_member);
        console.log("Completed remove group member\n\n\n")

        const check_removed = await josh.callSync('core', 'group_members', {group: 'QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC'});
        t.equal(JSON.stringify(check_removed), JSON.stringify({ Ok: '{"members":[]}' }));
        console.log("get group member by owner", check_removed);
        console.log("Completed get group members by owner\n\n\n"); 
});