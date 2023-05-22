# Users

---
## GET /users
---
    Get User List

* **URL Params**

    None

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
        ```
        [
            {<user_object>},
            {<user_object>},
            {<user_object>}
        ]
        ```

---
## GET /user/:user_id
---
    Gets a single User

* **URL Params**

    ***Required:***

        user_id: String

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
            
            {<data_object>>}
            
        
* **Error Response**
    * **Code:** 404
        
        **Content:**

            {
                "error": "Cannot find user with ID: [id]"
            }

---
## POST /user
---
    Creates a single new user

* **URL Params**

    None

* **Data Params**

        {
            "user_name": String
        }

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**

            {<data_object>>}

---
## PUT /user/:user_id
---
    Updates a single user

* **URL Params**

    ***Required:***

        user_id: String

* **Data Params**
    
        {
            "user_name": String
        }

* **Headers**

    None

* **Success Response**

    * **Code:** 200
        
* **Error Response**
    * **Code:** 404
        
        **Content:**

            {
                "error": "Cannot find user with ID: [id]"
            }

## DELETE /user/:user_id
---
    Deletes a single user

* **URL Params**

    ***Required:***

        user_id: String

* **Data Params**

        {
            "user_name": String
        }

* **Headers**

    None

* **Success Response**

    * **Code:** 200
        
* **Error Response**

    * **Code:** 404
        
        **Content:**
        ```
        {
            "error": "Cannot find user with ID: [id]"
        }
        ```

