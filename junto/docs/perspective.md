# Zome API Documentation
## Perspective

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