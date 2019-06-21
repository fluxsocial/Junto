const { Config, Container, Scenario } = require("@holochain/holochain-nodejs")
//const n3h = require('n3h');
Scenario.setTape(require('tape'))

const dnaPath = "./dist/junto-rust.dna.json"

const dna = Config.dna(dnaPath)
const agentEric = Config.agent("eric");
const agentJosh = Config.agent("josh")
const instanceJosh = Config.instance(agentJosh, dna)
const instanceEric = Config.instance(agentEric, dna);
const scenario = new Scenario([instanceJosh, instanceEric], {debugLog: true}) 

scenario.runTape('Can add and get users from perspective', async (t, {josh, eric}) => {
    //create user josh
    const register_result = await josh.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}});
    console.log("Register user result", register_result);
    t.equal(JSON.stringify(register_result), JSON.stringify({"Ok":{"private_den":{"address":"QmRhbdLQupJsE4NZajLCR2oCpCZjncoP656bh5TwXBTyHi","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Private"}},"shared_den":{"address":"Qmb3U3NGDvzr9H74yiXq1LZEwx5V5qrCivVWuk5jJhr4Mf","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Shared"}},"public_den":{"address":"Qmf4LcJ77idWGMPeGN1ngoqnmwot8tNmSHZ3mF1dZC8xsp","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Public"}},"pack":{"address":"QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC","entry":{"name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}},"profile":{"address":"QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","entry":{"username":"jdeepee"}},"user_perspective":{"address":"QmaAhrUjfAKVSoZRs6VUjEk3WWuzEXCTqQozWRFS4Au4mz","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Default Perspective"}}}}));
    console.log("Completed register profile\n\n\n");

    //create user eric
    const register_result_eric = await eric.callSync('core', 'create_user', {user_data: {username: "sunyatax", first_name: "Eric", last_name: "Yang", bio: "Junto Testing", profile_picture: "pictureurl"}});
    console.log("Register user eric result", register_result_eric);
    t.equal(JSON.stringify(register_result_eric), JSON.stringify({"Ok":{"private_den":{"address":"QmS1MKMD9s2Uxcx26TstJczRULv2fjuy5x2aztD4VHStFD","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Private"}},"shared_den":{"address":"Qmcq52qEYRsTVeirfR4sEPFRi6UxraHY8G5BGfESh9PMDf","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Shared"}},"public_den":{"address":"QmRW661xqgbR2wC31dFVKf2y95YiEGfDKZNfi5SZAGRCMR","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Public"}},"pack":{"address":"QmdZyapF7huQvjsB8tUw7riBXU1hoDDVYdG94qCgbvKpw8","entry":{"name":"Eric\'s Pack","owner":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","privacy":"Shared"}},"profile":{"address":"QmXF2BASNKjFg76hth4b6PJ4Btj6oeJLmn7AuZerCLrZiM","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","first_name":"Eric","last_name":"Yang","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}},"user_perspective":{"address":"QmWR9cLyApeJvZWoMgFcnZkqu6tVJLVH8DwsnMMzJBnCVc","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Default Perspective"}}}}));
    console.log("Completed register profile\n\n\n");

    //add user eric to josh's default perspective
    const add_user_to_perspective = await josh.callSync('core', 'add_user_to_perspective', {perspective: "QmaAhrUjfAKVSoZRs6VUjEk3WWuzEXCTqQozWRFS4Au4mz", target_user: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'});
    console.log("Add user to perspective result", add_user_to_perspective);
    t.equal(JSON.stringify(add_user_to_perspective), JSON.stringify({"Ok":"Qmbyh6p3AcmsuVkPNcJXTmE4xpqfRHtN1fg479GZXLGQFL"}));
    console.log('Completed add user to perspective');

    const perspective_users = await josh.callSync('core', 'get_perspectives_users', {perspective: "QmaAhrUjfAKVSoZRs6VUjEk3WWuzEXCTqQozWRFS4Au4mz"});
    console.log("User perspective results: ", perspective_users);
    t.equal(JSON.stringify(perspective_users), JSON.stringify({"Ok":[{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}}]}));
    console.log('Completed user perspective results');
})