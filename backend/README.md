# Setup Backend
## Install and setup rustc from [here](https://www.rust-lang.org/tools/install)
## Open project folder and run the following commands
- `rustup toolchain install nightly`
- `cd backend`
- Open PgAdmin and create a new Database
- `echo DATABASE_URL=postgres://username:password@localhost/database_name > .env`\
Enter details as per your configurations
- Add `C:\Program Files\PostgreSQL\14\lib` and `C:\Program Files\PostgreSQL\14\bin` to `Path` in environment variables. 
- `cargo install diesel_cli --no-default-features --features postgres` \
Doing this throws an error, go to `C:\Program Files\PostgreSQL\14\lib` copy `libpq.lib` and `libpq.dll` file into `C:\Users\{User Name}\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib` and also copy `libssl-1_1-x64.dll` from `C:\Users\tanis\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin` into the previously mentioned location \
Enter details as required
- This downloads diesel cli.
- Run `diesel migration run` and this does all the data feeding and querying.
- `cargo run` \
And you should have a running server at `http://127.0.0.1:8000`

# API Documentations
## Basic Information
- Base Url: `http://127.0.0.1:8000`
- Status Codes
  | Code |           Description |
  | :--- | --------------------: |
  | 200  |               Success |
  | 201  |               Created |
  | 204  |            No Content |
  | 400  |           Bad Request |
  | 401  |          Unauthorized |
  | 404  |             Not Found |
  | 500  | Internal Server Error |
## Basic Headers
```
{
    Global-Api-Key: "Token Value",
    Content-Type: application/json,
}
```
## All User API
- **Route:** /users/all
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:** None
- **Codes:** 
  - 200
- **Description:** Get all exisiting users information.
- **Example:**
  - Request: http://127.0.0.1:8000/users/all
  - Response:
    ```
    [
        {
            "id": 1,
            "name": "Superadmin 1",
            "role": "Superadmin",
            "email": "superadmin1@test.com",
            "password": "$2b$10$uJmFdn3aE5VV7fdM2GwwSOYyIbWvexp6Z0XvshZJ3qrmN141KJpDq",
            "api_key": "d2c2236f9f8361d7043b4c17d3de1de55e75779a26336b1d6b3abfe0027281fc"
        },
        {
            "id": 2,
            "name": "Admin 1",
            "role": "Admin",
            "email": "admin1@test.com",
            "password": "$2b$10$7Aqohk4M5h7YhE/9/R8zjuqZ5zbPOSaRmzT8PtRJ7PdgvnRTdtknq",
            "api_key": "c332856b12a28daf3b8f7c52db4dd0a75cc4179c33cde15ba4e4a74e9c9224ff"
        },
    ]
    ```

## Create User API
- **Route:** /users/create
- **Method:** POST
- **Data Required:**
  ```
  {
    "name": "Username",
    "role": "User/Admin",
    "email": "user@email.com",
    "password": "password",
    "api_key": ""
  }
  ```
- **Addtional Headers:**
  ```
  {
    Api-Key: "User Api Key"
  } 
  ```
- **Codes:** 
  - 201
  - 400
- **Description:** Register new users.
- **Example:**
  - Request: http://127.0.0.1:8000/users/create
  - Data: 
    ```
    {
        "name": "test1",
        "role": "User",
        "email": "email@gmail.com",
        "password": "user",
        "api_key": ""
    }
    ```
  - Response:
    ```
    {
        "id": 5,
        "name": "test1",
        "role": "User",
        "email": "email@gmail.com",
        "password": "$2b$10$AhrTVywUTk/Loz5fBbTV5.kCVsWlsOH3txnSiTBTXK0klEQ9P0pfO",
        "api_key": "$2b$10$TcKg7M8MRPJvXQQC1E2IouDfpQUQYnFTm1m2FKILNfYzGthkwfNCm"
    }
    ```

## User Details API
- **Route:** /users/<user_id>
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:** None
- **Codes:** 
  - 200
  - 404
- **Description:** Get specific user details by Id.
- **Example:**
  - Request: http://127.0.0.1:8000/users/2
  - Data: None
  - Response:
    ```
    {
        "id": 2,
        "name": "Admin 1",
        "role": "Admin",
        "email": "admin1@test.com",
        "password": "$2b$10$7Aqohk4M5h7YhE/9/R8zjuqZ5zbPOSaRmzT8PtRJ7PdgvnRTdtknq",
        "api_key": "c332856b12a28daf3b8f7c52db4dd0a75cc4179c33cde15ba4e4a74e9c9224ff"
    }
    ```

## Update User API
- **Route:** /users/update/<user_id>
- **Method:** POST
- **Data Required:**
  ```
  {
    "name": "new username",
    "role": "User/Admin"
  }
  ```
- **Addtional Headers:**
  ```
  {
    Api-Key: "User Api Key"
  } 
  ```
- **Codes:** 
  - 200
  - 404
  - 400
- **Description:** Update existing user data.
- **Example:**
  - Request: http://127.0.0.1:8000/users/update/4
  - Data: 
    ```
    {
        "name": "test65",
        "role": "User"
    }
    ```
  - Response:
    ```
    {
        "id": 4,
        "name": "test65",
        "role": "User",
        "email": "user1@test.com",
        "password": "$2b$10$ekxD3kTJvnQZ0YRGgMb5lOo.QHCFgO6Hn6kd9TlXeTYAqHCBhTPSy",
        "api_key": "b3333cfd5e7a2a9d707d9f600b79baab5875d8a3e8a550b1955b40c6a5d0cdd1"
    }
    ```

## Delete User API
- **Route:** /users/delete/<user_id>
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:**
  ```
  {
    Api-Key: "User Api Key"
  } 
  ```
- **Codes:** 
  - 204  
- **Description:** Delete existing user.
- **Example:**
  - Request: http://127.0.0.1:8000/users/delete/4
  - Data: None
  - Response:
    ```
    {}
    ```

## Login User API
- **Route:** /auth/login
- **Method:** POST
- **Data Required:**
    ```
    {
        "email":"user_email@test.com",
        "password": "password"
    }
    ```
- **Addtional Headers:** None
- **Codes:** 
  - 200
  - 401  
- **Description:** Authenticate user login.
- **Example:**
  - Request: http://127.0.0.1:8000/auth/login
  - Data:
    ```
    {
        "email":"superadmin1@test.com",
        "password": "superadmin"
    }
    ```
  - Response:
    ```
    {
        "id": 1,
        "name": "Superadmin 1",
        "role": "Superadmin",
        "email": "superadmin1@test.com",
        "password": "$2b$10$uJmFdn3aE5VV7fdM2GwwSOYyIbWvexp6Z0XvshZJ3qrmN141KJpDq",
        "api_key": "d2c2236f9f8361d7043b4c17d3de1de55e75779a26336b1d6b3abfe0027281fc"
    }
    ```

## Logout API
- **Route:** /auth/logout
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:**
  ```
  {
    Api-Key: "User Api Key"
  } 
  ```
- **Codes:**
  - 200
  - 401  
- **Description:** Verify logout of existing user login.
- **Example:**
  - Request: http://127.0.0.1:8000/auth/logout
  - Data: None
  - Response:
    ```
    {
        "message": "Logout Success"
    }
    ```

## All Products API
- **Route:** /products/all
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:** None
- **Codes:** 
  - 200
  - 400
- **Description:** Get all products information.
- **Example:**
  - Request: http://127.0.0.1:8000/products/all
  - Data: None
  - Response:
    ```
    [
        {
            "id": 1,
            "name": "Product 1",
            "price": 200,
            "description": "This is Product 1",
            "created_by": 2,
            "creator_name": "Admin 1",
            "creator_email": "admin1@test.com"
        },
        {
            "id": 2,
            "name": "Product 2",
            "price": 400,
            "description": "This is Product 2",
            "created_by": 3,
            "creator_name": "Admin 2",
            "creator_email": "admin2@test.com"
        },
    ]
    ```

## Create Product API
- **Route:** /product/create
- **Method:** POST
- **Data Required:** 
  ```
    {
        "name": "Product Name",
        "description": "Product Description",
        "price": Price(int),
        "created_by": Admin Id(int)
    }
  ```
- **Addtional Headers:**
  ```
  {
    Api-Key: "User Api Key"
  } 
  ```
- **Codes:** 
  - 201
  - 400
- **Description:** Create new product.
- **Example:**
  - Request: http://127.0.0.1:8000/product/create
  - Data:
    ```
    {
        "name": "Product 6",
        "description": "This is new product 6",
        "price": 450,
        "created_by": 3
    }
    ```
  - Response:
    ```
    {
        "id": 7,
        "name": "Product 6",
        "price": 450,
        "description": "This is new product 6",
        "created_by": 3
    }
    ```

## Product Details API
- **Route:** /products/<product_id>
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:** None
- **Codes:** 
  - 200
  - 404
- **Description:** Get product details by Id.
- **Example:**
  - Request: http://127.0.0.1:8000/products/4
  - Data: None
  - Response:
    ```
    {
        "id": 4,
        "name": "Product 4",
        "price": 300,
        "description": "This is Product 4",
        "created_by": 2,
        "creator_name": "Admin 1",
        "creator_email": "admin1@test.com"
    }
    ```

## Update Product API
- **Route:** /products/update/<product_id>
- **Method:** POST
- **Data Required:**
  ```
  {
    "name": "New Product Name",
    "price": New price(int),
    "description": "New product Description"
  }
  ```
- **Addtional Headers:**
  ```
  {
    Api-Key: "User Api Key"
  } 
  ```
- **Codes:** 
  - 200
  - 404
- **Description:** Update existing product information by Id.
- **Example:**
  - Request: http://127.0.0.1:8000/products/update/8
  - Data: 
    ```
    {
        "name": "Product 6 Updated",
        "price": 450,
        "description": "This is new product 6"
    }
    ```
  - Response:
    ```
    {
        "id": 8,
        "name": "Product 6 Updated",
        "price": 450,
        "description": "This is new product 6",
        "created_by": 3
    }
    ```

## Delete Product API
- **Route:** /products/delete/8
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:**
  ```
  {
    Api-Key: "User Api Key"
  } 
  ```
- **Codes:** 
  - 204  
- **Description:** Delete existing product.
- **Example:**
  - Request: http://127.0.0.1:8000/products/delete/4
  - Data: None
  - Response:
    ```
    {}
    ```


## Search Product By Name API
- **Route:** /products/search/<query_name>
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:** None
- **Codes:** 
  - 200
  - 404
- **Description:** Filter products by name.
- **Example:**
  - Request: http://127.0.0.1:8000/product/search/product
  - Data: None
  - Response:
    ```
    [
        {
            "id": 1,
            "name": "Product 1",
            "price": 200,
            "description": "This is Product 1",
            "created_by": 2,
            "creator_name": "Admin 1",
            "creator_email": "admin1@test.com"
        },
    ]
    ```

## Search Product By Creator Id API
- **Route:** /products/search/creator/<user_id>
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:** None
- **Codes:** 
  - 200
  - 404
- **Description:** Filter products by creator Id/admin Id.
- **Example:**
  - Request: http://127.0.0.1:8000/product/search/creator/2
  - Data: None
  - Response:
    ```
    [
        {
            "id": 1,
            "name": "Product 1",
            "price": 200,
            "description": "This is Product 1",
            "created_by": 2,
            "creator_name": "Admin 1",
            "creator_email": "admin1@test.com"
        },
        {
            "id": 4,
            "name": "Product 4",
            "price": 300,
            "description": "This is Product 4",
            "created_by": 2,
            "creator_name": "Admin 1",
            "creator_email": "admin1@test.com"
        }
    ]
    ```

## Search Product By Creator Name API
- **Route:** /products/search/creator/name/<query_name>
- **Method:** GET
- **Data Required:** None
- **Addtional Headers:** None
- **Codes:** 
  - 200
  - 404
- **Description:** Filter products by creator name.
- **Example:**
  - Request: http://127.0.0.1:8000/product/search/creator/name/admin 1
  - Data: None
  - Response:
    ```
    [
        {
            "id": 1,
            "name": "Product 1",
            "price": 200,
            "description": "This is Product 1",
            "created_by": 2,
            "creator_name": "Admin 1",
            "creator_email": "admin1@test.com"
        },
        {
            "id": 4,
            "name": "Product 4",
            "price": 300,
            "description": "This is Product 4",
            "created_by": 2,
            "creator_name": "Admin 1",
            "creator_email": "admin1@test.com"
        }
    ]
    ```

