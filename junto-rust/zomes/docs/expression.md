# Zome API Documentation

## Expression

**Post Expression**
###### Request: 
```
Endpoint: /post_expression
Arguments: { expression: {expression object data}, channels: [array of channels], context: [array of context(s) addresses] }
```

###### Response: 
```
Success: {Ok: "address of post"}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Make Resonation**
###### Request: 
```
Endpoint: /resonation
Arguments: { expression: 'address of expression to resonate' }
```

###### Response:
```
Success: {Ok: "Resonation Generated"}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Get Expression**
###### Request: 
```
Endpoint: /get_expressions
Arguments: { query_root: "address of query root", query_string: "query string to build expression query", query_options: QueryOptions, context: "address of context in which expression exists", target_type: QueryTarget, query_type: QueryType) }
```

###### Response:
```
Success: {Ok: [{ "entry": { expression entry data }, "address": "address of expression }, ...]}
Error: {Err: {Error Type: 'Error Message')}}
```

**Other Notes**:
The get_expressions function has a few arguments which might not be easily understandable as to what they do. I will go over each argument here detailing how to use the argument and how they effect the results. Please note this is the first iteration of trying to handle our more advance query functionality into a Holochain application. It is likely this will be refined in the future as we become more comfortable with Holochain/Rust and the optimal designs start to reveal themselves.

**Query Root**: This is the root object where you want to begin the query. In most cases this will either be a channel object or time object. In Holochain all entries are stored "independently" (not related) to each other, in order to create relations between entries you use links. Thus if you want to get expressions you need to start at a given entry and then get links from here.

**Query String**: This is the parameter which defines how your query will be run in the function. The query parameter should be in the following format: "query_object_value<query_object_type>:query_object_value2<query_object_type2>:... (and so on for each parameter)". Each parameter will confine expression result(s) to only those objects. 

**Query Options**: QueryOptions is a Rust Enum - the parameters for this enum are as follows: FilterPopular, FilterNew, FilterOld. Any of these enum value(s) should be passed as a string exactly as seen above. Other enum value in subsequent arguments should also be passed as strings exactly matching the enum variant.

**Context**: Context outlines in which the expressions which you are searching for exist. There cases where this will be used are as follows: Searching globally (passing the hash of the DNA). Searching within a Den (passing the hash of a den). Searching within a Group (passing the hash of the group). 

**Target Type**: TargetType is a Rust Enum - the parameters for this enum are as follows: ExpressionPost, User. This argument details what kind of "expression" you wish to retrieve. If you want to retrieve an ExpressionPost or a User(s) profile, these are the only current supported returned expression types - if it proves beneficial I can add searching for groups/channels etc in this same function or perhaps this can just go into another function.

**Query Type**: QueryType is a Rust Enum - the parameters for this enum are as follows: And, Or. This argument details if the query string should be searching using and/or. And signifies that the function will get all expressions which are present in __every__ query paramter. Or signifies that it will get expression from __each__ query parameter no matter if it is present in other query parameters or not.

**Example**: If you are using get_expressions to search for a user. You have two possible options which may not be readily apparent. You can either pass a user object into the query parameter which will do a regular user search using the username. Or you can pass other parameters (and no username parameter); this will do a traditional ExpressionPost search but then get all ExpressionPost owners and then return these users. This is an interesting way to search for users, an example: being able to search via channels/time and then filter this using QueryOptions to see which user is a high contributor for any given time period/channel.