# crud_rust_api
## Live demo 
hosted on my Ubuntu virtual machine [Live Demo](http://13.81.250.86:8080/api)
on postman to test the endpoints

Test Like this
``` 
curl -X 'POST' \
    'http://13.81.250.86:8080/api' \
    -H 'accept: application/json' \
    -H 'Content-Type: application/json' \
    -d '{
    "name": "Odunayo",
    "email": "amatthew@gmail.com"
}'
```


## ER diagram
```
+----------------+            +--------------+
|     person     |            |    person    |
+----------------+            +--------------+
|   id (PK)      |            |     id (PK)  |
|   name         |            |     name     |
|   email        |            |     email    |
+----------------+            +--------------+
```

To run your Rust application locally using Docker Compose and test the endpoints using Postman, you can follow these steps and create a README file for your project:

## Project Setup and Running Locally

### Prerequisites
- Docker, Rustup/Cargo
- Docker Compose
- Postman (for testing)

### Step 1: Clone the Repository
```bash
git clone <repository_url>
cd <project_directory>
```

### Step 2: Build the Docker Image
Run the following command to build the Docker image for your Rust application:

```bash
docker-compose build
```

### Step 3: Start the Application
Start your Rust application and PostgreSQL database using Docker Compose:

```bash
docker-compose up rustapp
```

### Step 4: Testing Endpoints
Now that your application is running, you can use Postman to test the endpoints.

- Open Postman.
- Create a new request and set the request type (GET, POST, PUT, DELETE) and URL according to the endpoints you want to test. For example, if you want to test the GET request to retrieve all users, set the URL to `http://localhost:8080/api`.

Remember to adapt the URLs and request types to match the specific endpoints you want to test based on your application's API design.

### Step 5: Stop and Clean Up
To stop the containers and clean up, run:

```bash
docker-compose down
```
