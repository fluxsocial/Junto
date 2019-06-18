function registerAgent(agent, username, first_name, last_name) {
    const register_result = agent.call('core', 'create_user', {user_data: {username: username, first_name: first_name, last_name: last_name, bio: "Junto Testing", profile_picture: "pictureurl"}})
    //t.equal(JSON.stringify(register_result), JSON.stringify());
    return register_result
}

module.exports = {
    registerAgent: registerAgent
}