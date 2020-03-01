# Zome API Documentation

## Expression

**Post Expression**
###### Request: 
```
Endpoint: /post_expression
Arguments: { expression: {expression object data}, attributes: [array of attributes (channels)], context: [array of context(s) addresses] }
```

###### Response: 
```
Success: {Ok: "address of post"}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Post Comment Expression**
###### Request: 
```
Endpoint: /post_comment_expression
Arguments: { expression: {expression object data}, parent_expression: Address-of-parent }
```

###### Response: 
```
Success: {Ok: "address of post"}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Post Resonation**
###### Request: 
```
Endpoint: /post_resonation
Arguments: { expression: 'address of expression to resonate' }
```

###### Response:
```
Success: {Ok: "Resonation Generated"}
Error: {Err:  {Error Type: 'Error Message'} }
```

**Query Expressions**
###### Request: 
```
Endpoint: /query_expressions
Arguments: { perspective: "string of perspective type", attributes: ["attribute-to-query-with", ...], query_options: QueryOptions, target_type: QueryTarget, query_type: QueryType, dos: dos-u32, seed: "string-to-randomize-searching" }
```

###### Response:
```
Success: {"Ok":[
            {
                "expression": {
                                "address":"expression-address",
                                "entry":{
                                    "expression_type":"expression-type",
                                    "expression":{"expression-entry}
                                }
                },
                "sub_expressions":[
                                {
                                    "expression": {
                                                    "address":"expression-address",
                                                    "entry":{
                                                        "expression_type":"expression-type",
                                                        "expression":{"expression-entry}
                                                    }
                                    },
                                    "sub_expressions":[],
                                    "author_username":{"address":"username-address","entry":{"username":"username"}},
                                    "author_profile": {
                                                        "address":"Qmaao8yPQtLA7Muo8xxJFCvYFKf7m1HbNzrhtN9JUPHeiv",
                                                        "entry": {
                                                            "parent":"parent-address",
                                                            "first_name":"first-name",
                                                            "last_name":"last-name",
                                                            "bio":"bio",
                                                            "profile_picture":"pictureurl",
                                                            "verified":true
                                                        }
                                                    },
                                    "resonations":[],
                                    "timestamp":"timestamp",
                                    channels":[{"address": "channel-address","entry":{"value":"channel-value","attribute_type":"Channel"}}, ...]
                                },
                                ...
                ],
                "author_username":{"address":"username-address","entry":{"username":"username"}},
                "author_profile": {
                                    "address":"Qmaao8yPQtLA7Muo8xxJFCvYFKf7m1HbNzrhtN9JUPHeiv",
                                    "entry": {
                                        "parent":"parent-address",
                                        "first_name":"first-name",
                                        "last_name":"last-name",
                                        "bio":"bio",
                                        "profile_picture":"pictureurl",
                                        "verified":true
                                    }
                                },
                "resonations":[],
                "timestamp":"timestamp",
                channels":[{"address": "channel-address","entry":{"value":"channel-value","attribute_type":"Channel"}}, ...]
            }
        ]
    }
Error: {Err: {Error Type: 'Error Message')}}
```

**Other Notes**:
The get_expressions function has a few arguments which might not be easily understandable as to what they do. I will go over each argument here detailing how to use the argument and how they effect the results. Please note this is the first iteration of trying to handle our more advance query functionality into a Holochain application. It is likely this will be refined in the future as we become more comfortable with Holochain/Rust and the optimal designs start to reveal themselves.

**Perspective**: String which outlines which perspective to view posts from. This can be any of the following values: "dos", "random" or "address". Dos tells the function to make degree of seperation query. Random tells function to make a random query for posts. Otherwise it should be an address: this address should be for a user created perspective.

**Attributes**: This is the parameter which defines how your query will be run in the function. The query parameter should be in the following format: ["query_value<query_type>", ...] (and so on for each parameter)". Each parameter will confine expression result(s) to only those objects. 

**Query Options**: QueryOptions is a Rust Enum - the parameters for this enum are as follows: FilterPopular, FilterNew, FilterOld. Any of these enum value(s) should be passed as a string exactly as seen above. Other enum value in subsequent arguments should also be passed as strings exactly matching the enum variant.

**Context**: Context outlines in which the expressions which you are searching for exist. There cases where this will be used are as follows: Searching globally (passing the hash of the DNA). Searching within a Den (passing the hash of a den). Searching within a Group (passing the hash of the group). 

**Target Type**: TargetType is a Rust Enum - the parameters for this enum are as follows: ExpressionPost, User. This argument details what kind of "expression" you wish to retrieve. If you want to retrieve an ExpressionPost or a User(s) profile, these are the only current supported returned expression types - if it proves beneficial I can add searching for groups/channels etc in this same function or perhaps this can just go into another function.

**Query Type**: QueryType is a Rust Enum - the parameters for this enum are as follows: And, Or. This argument details if the query string should be searching using and/or. And signifies that the function will get all expressions which are present in __every__ query paramter. Or signifies that it will get expression from __each__ query parameter no matter if it is present in other query parameters or not.

**Dos**: Degree of seperation: value to determine how many pack layers deep the DOS query functionality will go. Must be between 1-6.

**Seed**: This is a string which will be used to help randomize (or return the same) results for DOS & random queries. This can be a string of literaly anything - but you most likley want some degree of randomness. Taking the current time in milliseconds since a given date would be an example of a "good enough" random string.
 
**Example**: If you are using get_expressions to search for a user. You have two possible options which may not be readily apparent. You can either pass a user object into the query parameter which will do a regular user search using the username. Or you can pass other parameters (and no username parameter); this will do a traditional ExpressionPost search but then get all ExpressionPost owners and then return these users. This is an interesting way to search for users, an example: being able to search via channels/time and then filter this using QueryOptions to see which user is a high contributor for any given time period/channel.