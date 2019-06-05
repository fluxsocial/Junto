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
Success: {Ok: {
    private_den: {address: address, entry: {"parent":"parent object (user address)", "name": "den_name", "privacy": "Private", "channel_type": "Den"}},
    shared_den: {address: address, entry: {"parent":"parent object (user address)", "name": "den_name", "privacy": "Shared", "channel_type": "Den"}},
    public_den: {address: address, entry:  {"parent":"parent object (user address)", "name": "den_name", "privacy": "Public", "channel_type": "Den"}},
    pack: {address: address, entry: {"parent": "parent object(user address)", "name": "pack name", "owner": "user address", "privacy": "Shared"}},
    profile:  {address: address, entry:{ parent: 'parent object (user address)', first_name: 'first_name', last_name: 'last_name', bio: 'bio', profile_picture: 'profile_picture',verified: true/false }},
    username: {address: address, entry: { username: 'username' }},
    user_perspective: {address: address, entry: {parent: 'parent object (user address)',"name": 'perspective_name', 'privacy':'Private', 'channel_type': 'Perspective'}}
    }
}
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

**Get User Profile From Username Address**
###### Request: 
```
Endpoint: /get_user_profile_from_address
Arguments: { username_address: "address of user"}
```

###### Response: 
```
Success: { Ok: { 'address': 'address-of-profile', entry: { parent: 'parent object (user address)', first_name: 'first_name', last_name: 'last_name', bio: 'bio', profile_picture: 'profile_picture',verified: true/false} } }
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
Success: { Ok: { 'address': 'address-of-profile', entry: { parent: 'parent object (user address)', first_name: 'first_name', last_name: 'last_name', bio: 'bio', profile_picture: 'profile_picture',verified: true/false} } }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get Username From Agent**
###### Request: 
```
Endpoint: /get_user_username_by_agent_address
Arguments: {}
```

###### Response: 
```
Success: { Ok: { 'address': 'address-of-username', 'entry': { 'username': 'username' } } }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Create Perspective**
###### Request:
```
Endpoint: /create_perspective
Arguments: {name: "name_of_perspective"}
```

###### Response:
```
Success: { Ok: {address: address, entry: {parent: 'parent object (user address)',"name": 'perspective_name', 'privacy':'Private', 'channel_type': 'Perspective'} } }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Add User To Perspective**
###### Request:
```
Endpoint: /add_user_to_perspective
Arguments: {perspective: 'address_of_perspective_to_use', target_user: 'address_of_user_to_add'}
```

###### Response:
```
Success: { Ok: 'address_of_link' }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get Users In Perspective**
###### Request:
```
Endpoint: /get_perspectives_users
Arguments: {perspective: 'address_of_perspective_to_query'}
```

###### Response:
```
Success: {Ok: [ {'address': 'address_of_user','entry': {'username': 'username_of_user'} } ] }
Error: {Err:  {Error Type: 'Error Message'} }
```