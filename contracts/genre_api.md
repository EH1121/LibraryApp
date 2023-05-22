
# Genre

## GET /genre/:user_id
----
    Gets genre list, Can be specified to only grab one genre

* **URL Params**

    ***Required:***

        user_id: String

    ***Optional:*** 
    
        genre: String

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
            
            [
                {
                    "index": String (UserID.Genre),
                    "docs_count": String,
                    "primary_size": String
                },
                ...
            ]
* **Error Response**
    * **Code:** 404
        
        **Content:**
        
            {
                "error": "Cannot find genre: [genre]"
            }

        OR

            {
                "error": "Cannot find user with ID: [user_id]"
            }

## POST /genre/:user_id
----
    Inserts a new genre into User

* **URL Params**

    **Required:**

        user_id: String

* **Data Params**

        {
            "genre": String
        }

* **Headers**

    None

* **Success Response**
    * **Code:** 201

* **Error Response**

    * **Code:** 404

        **Content:**

            {    
                "error": "Cannot find user with ID: [user_id]"
            }

    * **Code:** 409

        **Content:**

            {    
                "error": Genre already exist: [genre]
            }

## DELETE /genre/:user_id/:genre
----
    Deletes an index

* **URL Params**

    **Required:**

        user_id: String

        genre: String

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200

* **Error Response**
    * **Code:** 404

        **Content:**

            {
                "error": "Cannot find genre: [genre]"
            }

        OR

            {
                "error": "Cannot find user with ID: [user_id]"
            }