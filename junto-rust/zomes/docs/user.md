# Zome API Documentation

## User

**Create User**
###### Request: 
```
Endpoint: /create_user
Arguments: { user_data: {username: "username", first_name: "first_name", last_name: "last_name", profile_picture: "profile picture url", bio: "bio"} }
```

###### Response: 
```
Success: {Ok: "address of user"}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Dens**
###### Request: 
```
Endpoint: /user_dens
Arguments: { username_address: "address of user"}
```

###### Response: 
```
Success: { Ok: { 
    private_den: {"address": "den address", "entry": {"parent":"parent object (user address)", "name": "den_name", "privacy": "Private", "channel_type": "Den"}},
    shared_den: {"address": "den address", "entry": {"parent":"parent object (user address)", "name": "den_name", "privacy": "Shared", "channel_type": "Den"}},
    public_den: {"address": "den address", "entry": {"parent":"parent object (user address)", "name": "den_name", "privacy": "Public", "channel_type": "Den"}}, 
    } 
}

Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Pack**
###### Request: 
```
Endpoint: /user_pack
Arguments: { username_address: "address of user"}
```

###### Response: 
```
Success: { Ok: {"address": "pack address", "entry": {"parent": "parent object(user address)", "name": "pack name", "owner": "user address", "privacy": "Shared"}}}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get Username From Address**
###### Request: 
```
Endpoint: /get_username_from_address
Arguments: { username_address: "address of user"}
```

###### Response: 
```
Success: { Ok: { username: 'username' } }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get Profile From Address**
###### Request: 
```
Endpoint: /get_profile_from_address
Arguments: { username_address: "address of user"}
```

###### Response: 
```
Success: { Ok: { parent: 'parent object (user address)', first_name: 'first_name', last_name: 'last_name', bio: 'bio', profile_picture: 'profile_picture',verified: true/false } }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Profile From Agent**
###### Request: 
```
Endpoint: /get_user_profile_by_agent_address
Arguments: {}
```

###### Response: 
```
Success: { Ok: { parent: 'parent object (user address)', first_name: 'first_name', last_name: 'last_name', bio: 'bio', profile_picture: 'profile_picture',verified: true/false } }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Profile Address From Agent**
###### Request: 
```
Endpoint: /get_user_profile_address_by_agent_address
Arguments: {}
```

###### Response: 
```
Success: { Ok: 'user profile address'}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Name From Agent**
###### Request: 
```
Endpoint: /get_user_username_by_agent_address
Arguments: {}
```

###### Response: 
```
Success: { Ok: { username: 'username' } }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Name Address From Agent**
###### Request: 
```
Endpoint: /get_user_username_address_by_agent_address
Arguments: {}
```

###### Response: 
```
Success: { Ok: 'username address' }
Error: {Err:  {Error Type: 'Error Message'} }
```