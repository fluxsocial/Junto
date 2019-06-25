async function registerAgent(t, agent, username, first_name, last_name) {
    const register_result = await agent.call('core', 'create_user', {user_data: {username: username, first_name: first_name, last_name: last_name, bio: "Junto Testing", profile_picture: "pictureurl"}})
    console.log("Registered user: ", first_name);
    t.deepEqual(register_result.hasOwnProperty("Ok"), true);
    console.log("Completed user registration\n\n\n\n");
    return register_result
}

async function postExpression(t, agent, expression, attributes, context) {
    //Post expression to one context (global) with all four attributes specified - all unique with one tag having an uppercase letter
    const post_expression = await agent.call('core', 'post_expression', {expression: expression, attributes: attributes, context: context});
    console.log("Post expression result", post_expression);
    t.deepEqual(post_expression.hasOwnProperty("Ok"), true);
    console.log("Completed posting expression\n\n\n\n");
    return post_expression
}

async function postComment(t, agent, expression, parent_expression) {
    const comment_expression = await agent.call('core', 'post_comment_expression', {expression: expression, parent_expression: parent_expression});
    console.log("Comment expression result", comment_expression);
    t.deepEqual(comment_expression.hasOwnProperty("Ok"), true);
    console.log("Completed comment expression\n\n\n\n");
    return comment_expression
}

async function updateBitPrefix(t, agent, bit_prefix) {
    const update_bit_prefix_value = await agent.call('core', 'update_bit_prefix', {bit_prefix: bit_prefix});
    console.log("Update bit prefix result", update_bit_prefix_value);
    t.deepEqual(update_bit_prefix_value.hasOwnProperty("Ok"), true);
    console.log("Completed bit prefix config setting\n\n\n\n");
    return update_bit_prefix_value
}

async function queryExpressions(t, agent, perspective, attributes, query_options, target_type, query_type, dos, seed) {  
    const query = await agent.call('core', 'query_expressions', {perspective: perspective, 
                                                                    attributes: attributes,
                                                                    query_options: query_options,
                                                                    target_type: target_type,
                                                                    query_type: query_type,
                                                                    dos: dos,
                                                                    seed: seed});
    console.log("Make query result", query);
    t.deepEqual(query.hasOwnProperty("Ok"), true);
    console.log("Completed query\n\n\n\n");
    return query
}

async function getExpression(t, agent, address) {
    const get = await agent.call('core', 'get_expression', {expression: address});
    console.log("Get expression result", get);
    t.deepEqual(get.hasOwnProperty("Ok"), true);
    console.log("Completed get expression\n\n\n\n");
    return get
}

function getCurrentTimestamps() {
    let d = new Date();
    let year = d.getFullYear();
    let month = d.getUTCMonth() + 1;
    let day = d.getUTCDate();
    let hour = d.getUTCHours();  
    return {year: year, month: month, day: day, hour: hour}
}

async function addUserToPerspective(t, agent, perspective, target_user){
    const add_user_to_perspective = await agent.call('core', 'add_user_to_perspective', {perspective: perspective, target_user: target_user});
    console.log("Add user to perspective result", add_user_to_perspective);
    t.deepEqual(add_user_to_perspective.hasOwnProperty("Ok"), true);
    console.log('Completed add user to perspective\n\n\n\n');
    return add_user_to_perspective
}

async function isCollectionOwner(t, agent, den, user){
    //check current user is den owner
    const owner_status = await agent.call('core', 'is_collection_owner', {den: den, user: user});
    console.log("Get den(s) result", owner_status);
    t.deepEqual(owner_status.hasOwnProperty("Ok"), true);
    console.log("Completed is collection owner\n\n\n\n");
    return owner_status
}

async function isGroupMember(t, agent, group, user) {
    const is_group_member = await agent.call('core', 'is_group_member', {group: group, user: user});
    console.log("is group member result", is_group_member);
    t.deepEqual(is_group_member.hasOwnProperty("Ok"), true);
    console.log("Completed is group member\n\n\n\n");
    return is_group_member
}

async function getGroupMembers(t, agent, group, expect_ok) {
    const get_group_members = await agent.call('core', 'group_members', {group: group});
    console.log("Get group members", get_group_members);
    if (expect_ok == true){
        t.equal(get_group_members.hasOwnProperty("Ok"), true);
    } else {
        t.equal(get_group_members.hasOwnProperty("Err"), true);
    };
    console.log("Completed get group members by owner\n\n\n\n");
    return get_group_members
}

async function addPackMember(t, agent, user) {
    const add_group_member = await agent.call('core', 'add_pack_member', {username_address: user});
    console.log("add group member result", add_group_member);
    t.deepEqual(add_group_member.hasOwnProperty("Ok"), true);
    console.log("Completed add group member\n\n\n\n");
    return add_group_member
}

async function getUserPack(t, agent, user) {
    const get_pack = await agent.call('core', 'user_pack', {username_address: user});
    console.log("Get pack result", get_pack);
    t.deepEqual(get_pack.hasOwnProperty("Ok"), true);
    console.log("Completed get pack\n\n\n\n");
    return get_pack

}

async function removeGroupMember(t, agent, user, group) {
    const remove_group_member = await agent.call('core', 'remove_group_member', {username_address: user, group: group});
    console.log("remove group member result", remove_group_member);
    t.deepEqual(remove_group_member.hasOwnProperty("Ok"), true);
    console.log("Completed remove group member\n\n\n\n")
    return remove_group_member
}

async function getHolochainEnv(t, agent) {
    const holochain_env = await agent.call('core', 'show_env', {});
    console.log("Show env result, holochain_env", holochain_env);
    t.deepEqual(holochain_env.hasOwnProperty("Ok"), true);
    console.log("Completed get Holochain Env\n\n\n\n");
    return holochain_env
}

async function getDens(t, agent, user) {
    const get_dens = await agent.call('core', 'user_dens', {username_address: user});
    console.log("Get den(s) result", get_dens);
    t.equal(get_dens.hasOwnProperty("Ok"), true);
    console.log("Completed get den results\n\n\n\n");
    return get_dens
}

async function getPerspectivesUsers(t, agent, perspective) {
    const perspective_users = await agent.call('core', 'get_perspectives_users', {perspective: perspective});
    console.log("User perspective results: ", perspective_users);
    t.equal(perspective_users.hasOwnProperty("Ok"), true);
    console.log('Completed user perspective results\n\n\n\n');
    return perspective_users
}

module.exports = {
    registerAgent: registerAgent,
    postExpression: postExpression,
    postComment: postComment,
    updateBitPrefix: updateBitPrefix,
    queryExpressions: queryExpressions,
    getExpression: getExpression,
    getCurrentTimestamps: getCurrentTimestamps,
    addUserToPerspective: addUserToPerspective,
    isCollectionOwner: isCollectionOwner,
    isGroupMember: isGroupMember,
    addPackMember: addPackMember,
    getUserPack: getUserPack,
    removeGroupMember: removeGroupMember,
    getHolochainEnv: getHolochainEnv,
    getDens: getDens,
    getGroupMembers: getGroupMembers,
    getPerspectivesUsers: getPerspectivesUsers
}