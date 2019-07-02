# Zome API Documentation
## Collection

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