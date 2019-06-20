const { Config, Container, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require('tape'))

const dnaPath = "./dist/junto-rust.dna.json"

const dna = Config.dna(dnaPath)
const agentJosh = Config.agent("josh")
const instanceJosh = Config.instance(agentJosh, dna)
const scenario = new Scenario([instanceJosh], {debugLog: false}) 

scenario.runTape('Can post expression and do basic random query', async (t, {josh}) => {
    const register_result = await josh.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}});
    console.log("Register user result", register_result);
    t.equal(JSON.stringify(register_result), JSON.stringify({"Ok":{"private_den":{"address":"QmV7H3Mhpdpj9NfFq2pgwzRd83uEjQupsHa5zwVVeCWSd2","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmV9j9LNfc4spvT8qNA24vjMjC4JEnoVfidfiBfnY4PUs3","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"Qmc48qWCdrCEqJVn1a4XZd6Eyrsu1W5jHHi1CgsVJgEAMx","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmU6oLeoZrQjNeT8kmpXRYn8U58FmqZ8rC6f7jr7tfMWKC","entry":{"name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}},"profile":{"address":"QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","entry":{"username":"jdeepee"}},"user_perspective":{"address":"QmcBgVN5mo8ACrX1Z1f2ZXNFzbRWSGhMskuNoJXe9fYQ71","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
    console.log("Completed register profile\n\n\n");

    const holochain_env = await josh.callSync('core', 'show_env', {});
    console.log("Show env result, holochain_env", holochain_env);
    const dna = holochain_env.Ok.dna_address;
    console.log("DNA of application: ", dna, "\n\n\n");

    const update_bit_prefix_value = await josh.callSync('core', 'update_bit_prefix', {bit_prefix: 1});
    console.log("Update bit prefix result", update_bit_prefix_value);
    t.equal(JSON.stringify(update_bit_prefix_value), JSON.stringify({ Ok: 1}));
    console.log("Completed bit prefix config setting");

    //Post expression to one context (global) with all four tags specified - all unique with one tag having an uppercase letter
    const post_global_expression = await josh.callSync('core', 'post_expression', {
        expression: 
                {
                    expression: {
                        ShortForm: {
                            background: "",
                            body: "This is the first test expression"
                        }
                    },
                    expression_type: "ShortForm"
                }, 
        tags: ["holochain", "Junto", "social", "holo"], 
        context: [dna]
    });
    console.log("Post expression 1 result", post_global_expression);
    t.equal(JSON.stringify(post_global_expression), JSON.stringify({"Ok":"QmT9LnUxYb6dBUpwvwfDnLTsDcKTAmKYqj9LHcW3ZWyyQW"}));
    console.log("Completed posting expression\n\n\n\n");

    let d = new Date();
    let year = d.getFullYear();
    let month = d.getUTCMonth() + 1;
    let day = d.getUTCDate();
    let hour = d.getUTCHours();    
    const random_query = await josh.callSync('core', 'get_expression', {perspective: "random", 
                                                                        query_points: ["social<tag>", "junto<tag>", "holochain<tag>", "holo<tag>", "jdeepee<user>", "ShortForm<type>", year+"<time:y>", "0"+month+"<time:m>", day+"<time:d>", hour+"<time:h>"],
                                                                        query_options: "FilterNew",
                                                                        target_type: "ExpressionPost",
                                                                        query_type: "And",
                                                                        dos: 1,
                                                                        seed: "otally random seed"});
    console.log("Random query result: ", random_query)
    t.equal(JSON.stringify(random_query), JSON.stringify({"Ok":[{"address":"QmT9LnUxYb6dBUpwvwfDnLTsDcKTAmKYqj9LHcW3ZWyyQW","entry":{"expression_type":"ShortForm","expression":{"ShortForm":{"background":"","body":"This is the first test expression"}}}}]}));
})