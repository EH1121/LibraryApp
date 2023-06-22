
# Document

## GET /book/:user_id/:genre/:book_id
----
    Returns a single document in an index
* **URL Params**

    ***Required:***

        user_id: String
        genre: String
        book_id: String

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200

        **Content:**

            {<data object>}

* **Error Response**
    * **Code:** 404

        **Content:**

            {    
                "error": "Cannot find user with ID: [user_id]"
            }

        OR

            {    
                "error": "Cannot find genre: [genre]"
            }

        OR

            {    
                "error": "Cannot find book with ID: [book_id]"
            }

## POST /search/:user_id
----
    Search all genres, Can search a specific genre

* **URL Params**

    ***Required:*** 

        user_id: String

    ***Optional:***

        genre: String

* **Data Params**

        {
            "search_term": String, (Optional)
            "search_fields": Comma Separated Fields, (Optional)
            "return_fields": Comma Separated Fields, (Optional)
            "from": int, (Optional)
            "count": int, (Optional)
        }
    

* **Headers**

    None

* **Success Response**
    * **Code:** 200

        **Content:**
        
            {
                "data" = 
                    [
                        {<data_object>},
                        {<data_object>},
                        {<data_object>}
                    ],
                "took": int,
                "total": int,
                "from": int,
                "count": int
            }
        
* **Error Response**
    * **Code:** 404

            {    
                "error": "Cannot find user with ID: [user_id]"
            }

        OR

        Content:
        
            {
                "error": "Cannot find genre: [genre]"
            }

    * **Code:** 400

        Content:
        
            {
                "error": "Bad Data Given"
            }
        

## GET /search/:user_id
----
    Search all genres, Can search a specific genre

* **URL Params**

    ***Required:*** 

        user_id: String

    ***Optional:***

        genre: String
        search_term: String, 
        search_fields: Comma Separated Fields, 
        return_fields: Comma Separated Fields, 
        from: int, 
        count: int

* **Data Params**

    None
    
* **Headers**

    None

* **Success Response**
    * **Code:** 200

        **Content:**
        
            {
                "data" = 
                    [
                        {<data_object>},
                        {<data_object>},
                        {<data_object>}
                    ],
                "took": int,
                "total": int,
                "from": int,
                "count": int
            }
        
* **Error Response**
    * **Code:** 404

            {    
                "error": "Cannot find user with ID: [user_id]"
            }

        OR

        Content:
        
            {
                "error": "Cannot find genre: [genre]"
            }

    * **Code:** 400

        Content:
        
            {
                "error": "Bad Data Given"
            }

## POST /book/:user_id/:genre
----
    Insert One or Multiple Books into a genre

* **URL Params**

    **Required:**

        user_id: String
        genre: String

* **Data Params**

        [
            {
                "isbn": String (Optional),
                "judul": String (Optional),
                "penulis": String (Optional),
                "penerbit": String (Optional),
                "genre": Vec<String> (Optional),
                "bahasa": String (Optional),
                "jumlah_halaman": usize (Optional),
                "tanggal_terbit": date (dd-MM-yyyy, Optional)
            },
            {data_object},
            ...
        ]

* **Headers**

    None

* **Response**
    * **Code:** 200
    
        **Content:**
        
            [
                {
                    "doc_num": int,
                    "error": String,
                    "code": StatusCode
                },
                ... (List is empty if there are no errors)
            ]
        
* **Error Response**
    * **Code:** 404
        
        **Content:**

        ```
        {
            "error": "Cannot find user with ID: [user_id]"
        }
        ```

        OR
        
        ```
        {
            "error": "Cannot find genre: [genre]"
        }
        ```

## PUT /book/:user_id/:genre/:book_id
----
    Update a single book

* **URL Params**
    
    ***Required:***

        user_id: String
        genre: String
        book_id: String

* **Data Params**

        {
            "isbn": String (Optional),
            "judul": String (Optional),
            "penulis": String (Optional),
            "penerbit": String (Optional),
            "genre": Vec<String> (Optional),
            "bahasa": String (Optional),
            "jumlah_halaman": usize (Optional),
            "tanggal_terbit": date (dd-MM-yyyy, Optional)
        }

* **Headers**

    None

* **Success Response**

    * **Code:** 200

* **Error Response**
    * **Code:** 400

        **Content**:
            {
                "error": "Bad Data Given"
            }

        OR

    * **Code:** 404

        **Content**:

            {    
                "error": "Cannot find user with ID: [user_id]"
            }

        OR

            {
                "error": "Cannot find genre: [genre]"
            }
    
## DELETE /book/:user_id/:genre/:book_id
----
    Delete a single book

* **URL Params**
    
    ***Required:***

        user_id: String
        genre: String
        book_id: String

* **Data Params**

    None

* **Headers**

    None

* **Success Response**

    * **Code:** 200

* **Error Response**

    * **Code:** 404

        Content:
        ```
        {    
            "error": "Cannot find user with ID: [user_id]"
        }
        ```

        OR

        ```
        {
            "error": "Cannot find genre: [genre]"
        }
        ```

        OR

        ```
        {
            "error": "Cannot find book with ID: [book_id]"
        }
        ```
