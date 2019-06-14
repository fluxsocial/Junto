const { Config, Container, Scenario } = require("@holochain/holochain-nodejs")

Scenario.setTape(require('tape'))

const dnaPath = "./dist/junto-rust.dna.json"

const dna = Config.dna(dnaPath)
const agent1 = Config.agent("agent1")
const agent2 = Config.agent("agent2")
const agent3 = Config.agent("agent3")
const agent4 = Config.agent("agent4")
const agent5 = Config.agent("agent5")
const agent6 = Config.agent("agent6")
const agent7 = Config.agent("agent7")
const instance1 = Config.instance(agent1, dna)
const instance2 = Config.instance(agent2, dna)
const instance3 = Config.instance(agent3, dna)
const instance4 = Config.instance(agent4, dna)
const instance5 = Config.instance(agent5, dna)
const instance6 = Config.instance(agent6, dna)
const instance7 = Config.instance(agent7, dna)
const scenario1 = new Scenario([instance1], {debugLog: true})
const scenario2 = new Scenario([instance2], {debugLog: false})
const scenario3 = new Scenario([instance3], {debugLog: false})
const scenario4 = new Scenario([instance4], {debugLog: false})
const scenario5 = new Scenario([instance5], {debugLog: false})
const scenario6 = new Scenario([instance6], {debugLog: false})
const scenario7 = new Scenario([instance7], {debugLog: false})
const scenario8 = new Scenario([instance1, instance2], {debugLog: false})

scenario1.runTape('Can register agent 1', async (t, {agent1}) => {
    let register_result = await agent1.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}});
    console.log("Signed up Josh");
    t.equal(JSON.stringify(register_result), JSON.stringify({"Ok":{"private_den":{"address":"QmV7H3Mhpdpj9NfFq2pgwzRd83uEjQupsHa5zwVVeCWSd2","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmV9j9LNfc4spvT8qNA24vjMjC4JEnoVfidfiBfnY4PUs3","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"Qmc48qWCdrCEqJVn1a4XZd6Eyrsu1W5jHHi1CgsVJgEAMx","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}},"profile":{"address":"QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","entry":{"username":"jdeepee"}},"user_perspective":{"address":"QmcBgVN5mo8ACrX1Z1f2ZXNFzbRWSGhMskuNoJXe9fYQ71","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
})

scenario2.runTape('Can register agent 2', async (t, {agent2}) => {
    let register_result_2 = await agent2.callSync('core', 'create_user', {user_data: {username: "sunyatax", first_name: "Eric", last_name: "Yang", bio: "Junto Testing", profile_picture: "pictureurl"}});
    t.equal(JSON.stringify(register_result_2), JSON.stringify({"Ok":{"private_den":{"address":"QmXi3gko95vvsYWhdTocjBPHTUZsBKnu9coZc4EJKMkwFe","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmTunjPizi21fTRU23WufNVMFAXLd1XRnYDab9nynrYgKD","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmUWkBB8ttdEUf6nEyMWodrJQcgv9fuQKFuRg7A6hKSj41","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmTevRrtjaaJzNCESubqfZfZjNXxJH4RxFBKFb9Nd7LUWh","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Pack","owner":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","privacy":"Shared"}},"profile":{"address":"QmXF2BASNKjFg76hth4b6PJ4Btj6oeJLmn7AuZerCLrZiM","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","first_name":"Eric","last_name":"Yang","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}},"user_perspective":{"address":"QmSxHAHwnE3Qw76uS6jviKMkVW1g3LCU828hxsqKqU6dpd","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
})

scenario3.runTape('Can register agent 3', async (t, {agent3}) => {
    let register_result_3 = await agent3.call('core', 'create_user', {user_data: {username: "db", first_name: "Donna", last_name: "", bio: "Junto Testing", profile_picture: "pictureurl"}});
    t.equal(JSON.stringify(register_result_3), JSON.stringify({"Ok":{"private_den":{"address":"QmS522FBoEyiKyXTAL2o2EcmFh5hk9VDxCY1orRBc6M8Nh","entry":{"parent":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","name":"Donna\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmV8ndMAkdinRNVJMmobhqz7ZwEvPE6pQi6FcxiSFmX5eA","entry":{"parent":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","name":"Donna\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmUz6sVUkTjtd7d59XBeBVuhTQrN8WWJZXwUyWjeg7qAx1","entry":{"parent":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","name":"Donna\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmSbvh8bNFXqjdy6vGwtQmkyN3tCv2p43wp2Zy8M6TDQ8b","entry":{"parent":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","name":"Donna\'s Pack","owner":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","privacy":"Shared"}},"profile":{"address":"QmYsS2D33ZcSaiQVzk3WRXBM59PZ6x8WhZScpeAnKxS7iu","entry":{"parent":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","first_name":"Donna","last_name":"","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","entry":{"username":"db"}},"user_perspective":{"address":"QmcxSvh3JmSHLdw9PXVV7YzcHwtFJfd4wpRaEkf18gYyPm","entry":{"parent":"Qma8a8SXgdCg2ZPpFYvjj4gXNq6A5fPChNo9MSYSrb3Zp8","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
})

scenario4.runTape('Can register agent 4', async (t, {agent4}) => {
    let register_result_4 = await agent4.call('core', 'create_user', {user_data: {username: "will", first_name: "Will", last_name: "", bio: "Junto Testing", profile_picture: "pictureurl"}});
    t.equal(JSON.stringify(register_result_4), JSON.stringify({"Ok":{"private_den":{"address":"QmY3k8bj8DwZvGfemHUtyQWE4sRam117tCVYk9FJuBLTER","entry":{"parent":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","name":"Will\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmRs4HLcjWAqGk9gL8aZ5JP2eT52Lv6W2Tc6nVAY6139yR","entry":{"parent":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","name":"Will\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmaypaLLCUQ5yeGHq6pR9Vqy8aQ6PHznjpbcDUQCGsNEQZ","entry":{"parent":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","name":"Will\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"Qmb398TWfhQxg3TvisLaF1p6EBcR2prJzcVhRPcgKht8Y2","entry":{"parent":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","name":"Will\'s Pack","owner":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","privacy":"Shared"}},"profile":{"address":"QmZggtZZHhUtZddH89Tq1XU24CPYSxw5voLxk4Hzkt1YtQ","entry":{"parent":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","first_name":"Will","last_name":"","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","entry":{"username":"will"}},"user_perspective":{"address":"Qmaf88KhLvJAWCKZGML3coTxq3DtgjAN8PfXjLavY4vE3d","entry":{"parent":"QmNSTpRSMCPQxKnYeBss9XqM3TaGUP4v3gh9qGzgbKj9y9","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
})

scenario5.runTape('Can register agent 5', async (t, {agent5}) => {
    let register_result_5 = await agent5.call('core', 'create_user', {user_data: {username: "adam", first_name: "Adam", last_name: "", bio: "Junto Testing", profile_picture: "pictureurl"}});
    t.equal(JSON.stringify(register_result_5), JSON.stringify({"Ok":{"private_den":{"address":"QmbwFimSDAKGA5A8NjSpDshWDmvaTftchdzFxRMqLpHQAT","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmSUXR9HhBztiv1Mh9mqrVkrLXDRo6P3op7G1rqUoemNf3","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmRFRvRgf61qhUiSpF1zJh3WimBq6NkaBA4g9RoS26a4r4","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmcTTLfEoPbjgHZhPbNTL1uQAHdhnAKUjN1QxuXLEwqzuh","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Pack","owner":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","privacy":"Shared"}},"profile":{"address":"QmaTaN7E3ykbDmoESFR7TQBHuoUdnb9pdGzUzvFuT9PG4c","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","first_name":"Adam","last_name":"","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","entry":{"username":"adam"}},"user_perspective":{"address":"QmS8nbwzqRcQtMKx4bASWbjzEc2fusWpeVt2iL2io1uQ6L","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
})

scenario6.runTape('Can register agent 6', async (t, {agent6}) => {
    let register_result_5 = await agent6.call('core', 'create_user', {user_data: {username: "adam", first_name: "Adam", last_name: "", bio: "Junto Testing", profile_picture: "pictureurl"}});
    t.equal(JSON.stringify(register_result_5), JSON.stringify({"Ok":{"private_den":{"address":"QmbwFimSDAKGA5A8NjSpDshWDmvaTftchdzFxRMqLpHQAT","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmSUXR9HhBztiv1Mh9mqrVkrLXDRo6P3op7G1rqUoemNf3","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmRFRvRgf61qhUiSpF1zJh3WimBq6NkaBA4g9RoS26a4r4","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmcTTLfEoPbjgHZhPbNTL1uQAHdhnAKUjN1QxuXLEwqzuh","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Adam\'s Pack","owner":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","privacy":"Shared"}},"profile":{"address":"QmaTaN7E3ykbDmoESFR7TQBHuoUdnb9pdGzUzvFuT9PG4c","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","first_name":"Adam","last_name":"","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","entry":{"username":"adam"}},"user_perspective":{"address":"QmS8nbwzqRcQtMKx4bASWbjzEc2fusWpeVt2iL2io1uQ6L","entry":{"parent":"QmQNrbJWgs3ggpkMiMrMmxNTJTNcBo6MKUsCCuHMQCbr1f","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
})

scenario7.runTape('Can register agent 7', async (t, {agent7}) => {
    let register_result_7 = await agent7.call('core', 'create_user', {user_data: {username: "jessy", first_name: "Jessy", last_name: "", bio: "Junto Testing", profile_picture: "pictureurl"}});
    t.equal(JSON.stringify(register_result_7), JSON.stringify({"Ok":{"private_den":{"address":"QmRMRnQYsJvuBHQDkA68kaR9MDy7yNBH8kuCogScHNL8Q8","entry":{"parent":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","name":"Jessy\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmXZbPxNfB7tKPMqaeQ6zhFzAS7eAqP18FVrDG1vdB5JVV","entry":{"parent":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","name":"Jessy\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmTCndp5qRvnAWDrVNyM6smo88xrMLZEhoZ79jJf3y6vZ7","entry":{"parent":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","name":"Jessy\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"Qmdg1HR1kabBxjLQwHvFhhaHbJyfcfrWBoyTJVB76DsnD2","entry":{"parent":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","name":"Jessy\'s Pack","owner":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","privacy":"Shared"}},"profile":{"address":"QmTvEoTPnsL7JtKGGc9Ax9iUsZfjqdrwsexdZdGXHN8T2X","entry":{"parent":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","first_name":"Jessy","last_name":"","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","entry":{"username":"jessy"}},"user_perspective":{"address":"QmVj2KttWkATqTUi5THunQ8ta79xQb8mWtX2WdFHb1mzy8","entry":{"parent":"QmbTux6tLeXk82LRFJPdCnQfXG8qi25sbNqynfMnricUjz","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
})

scenario2.runTape('add pack member', async (t, {agent2}) => {
    const add_pack_member = await agent2.callSync('core', 'add_pack_member', {username_address: 'QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn'}); //add eric to josh's group
    t.equal(JSON.stringify(add_pack_member), JSON.stringify({ Ok: { message: 'User added to group' } }));
    console.log("add group member result", add_pack_member);
    console.log("Completed add group member to eric's group\n\n\n");
})