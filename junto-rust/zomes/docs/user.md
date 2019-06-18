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

**Show ENV**
###### Request: 
```
Endpoint: /show_env
Arguments: {}
```

###### Response: 
```
Success: { Ok: { dna_name: 'Junto Holochain Application', dna_address: 'QmUr87mgBrEmcBBtBxbuzBSpVtfb6qEQA7i7VnS7Tm3BLT',
     agent_id:
      '{"nick":"josh","pub_sign_key":"HcSciOQAX8yc9e4az3VMCcZ3Rk7Kyth87YZSXM4VTx6nkrupMwbrS6HUai5789r"}',
     agent_address:
      'HcSciOQAX8yc9e4az3VMCcZ3Rk7Kyth87YZSXM4VTx6nkrupMwbrS6HUai5789r',
     cap_request:
      { cap_token: 'QmappKBkZhYBNqYChTasMKmCYLb4HHE54QQJLX2sNKmeqo',
        provenance: [Array] } } }
Error: {Err:  {Error Type: 'Error Message'} }
```