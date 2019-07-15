# Zome API Documentation
## Collection

**Create Collection*
###### Request:
```
Endpoint: /create_collection
Arguments: {collection: {collection-data}, collection_tag: "type-of-collection"}
```

###### Response:
```
Success: { Ok: "address-of-collection" }
Error: {Err:  {Error Type: 'Error Message'} }
```


**Is Collection Owner**
###### Request:
```
Endpoint: /is_collection_owner
Arguments: {collection: "address-of-collection", user: "address-of-user"}
```

###### Response:
```
Success: { Ok: true/false }
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get User Dens**
###### Request: 
```
Endpoint: /get_user_dens
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