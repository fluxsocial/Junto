# Zome API Documentation
## Group

**Add Pack Member**
###### Request: 
```
Endpoint: /add_pack_member
Arguments: { username_address: 'username address of user to add' }
```

###### Response: 
```
Success: {Ok: { "message": "User added to group" }}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Add Member To Group**
###### Request: 
```
Endpoint: /add_member_to_group
Arguments: { username_address: 'username address of user to add', group: 'address of group to add into' }
```

###### Response: 
```
Success: {Ok: { "message": "User added to group" }}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Remove Group Member**
###### Request: 
```
Endpoint: /remove_group_member
Arguments: { username_address: 'username address of user to remove', group: 'address of group to remove from' }
```

###### Response: 
```
Success: {Ok: { "message": "User remove from group" }}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Is Group Member**
###### Request: 
```
Endpoint: /is_group_member
Arguments: { username_address: 'username address of user to check', group: 'address of group' }
```

###### Response: 
```
Success: {Ok: true/false}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Is Group Owner**
###### Request: 
```
Endpoint: /is_group_owner
Arguments: { username_address: 'username address of user to check', group: 'address of group' }
```

###### Response: 
```
Success: {Ok: true/false}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get Group Members**
###### Request: 
```
Endpoint: /group_members
Arguments: { group: 'address of group' }
```

###### Response: 
```
Success: {Ok: { "members": [{"address": "address of user username", "entry": {"username": "username of user"}} ] } }
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
Success: { Ok: {"address": "pack address", "entry": {"name": "pack name", "owner": "user address", "privacy": "Shared"}}}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Member Packs**
###### Request: 
```
Endpoint: /get_user_member_packs
Arguments: { username_address: "address of user"}
```

###### Response: 
```
Success: { Ok: [
            {"address": "pack address", "entry": {"name": "pack name", "owner": "user address", "privacy": "Shared"}},
            ...
            ]
        }
Error: {Err:  {Error Type: 'Error Message'} }
```

