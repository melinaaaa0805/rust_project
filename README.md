# Project Documentation

## Introduction
This documentation provides instructions for downloading, building, and running the project developed in Rust. The project is an HTTP API that returns JSON-formatted request headers when a GET request is made to the "/ping" endpoint. Additionally, it handles other HTTP methods by responding with a 404 error and an empty response.

## Prerequisites
Before you can run the project, make sure you have the following prerequisites installed on your system:
- Rust
- Cargo

## Downloading the Project
You can download the project from its Git repository using the following command:

`git clone <repository-url> `

Replace <repository-url> with the URL of your Git repository.

## Building the Project
To build the project, follow these steps:

Navigate to the Project Directory: Open your terminal and navigate to the directory where you cloned the project using the cd command:

`cd project-directory`
Replace project-directory with the actual directory name where your project is located.

## Build the Project with Cargo: Once you are inside the project directory, build the project and compile its dependencies using Cargo:

`cargo build`
Cargo will automatically fetch and compile all the necessary dependencies for your project.

## Configuring the Port
By default, the project listens on port 3030. However, you can configure the listening port by setting the PING_LISTEN_PORT environment variable. For example, to run the project on port 8080:

`export PING_LISTEN_PORT=8080`
Replace 8080 with the desired port number.

## Running the Project
Start the project by executing the following command in your terminal:

`cargo run`
The project will start, and you should see log messages indicating that the server is up and running.

Sending Requests
To test the API, make GET requests using a tool like curl or a web browser to the following URL:

`http://localhost:8080/ping`
If you configured a different port, replace 3030 with the port number you specified.

Example using curl
You can use the curl command to make a GET request to the /ping endpoint:

`curl http://localhost:8080/ping`
You will receive a JSON response containing the request headers.

Handling Other HTTP Methods
The project responds with a 404 error and an empty response for any HTTP method other than GET when making a request to the root ("/") endpoint. For instance:

`curl -X POST http://localhost:8080/`
In this case, you will receive a 404 error response, as this project is specifically designed to handle GET requests to the "/ping" endpoint.

## Conclusion
Congratulations! You have successfully downloaded, built, and run the Rust project. You can now interact with the API by sending GET requests to the "/ping" endpoint. For any other HTTP method or endpoint, the project will respond with a 404 error.

Feel free to customize this documentation with project-specific details and additional information as needed.
