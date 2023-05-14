# password-manager-web-app
Password Manager Web App is a simple web application built using Rust that allows users to manage their passwords securely.

# Features
- create, edit, and delete passwords.
- generate strong passwords.
- store passwords securely using encryption.
- user authentication and authorization.
# Get started
To get started, clone this repository and run the following command to start the server:
```bash
$ cargo run
```
The server will start running on http://localhost:6966.

# API endpoints
### he following endpoints are available

## Env variable (.env file)
you dont actually need to set the .env file manually. The project has been set up thorugh .yaml files.
And the project will automatically detect the env(local/prod) accordingly.

## Dockerfile
You can also run the application using Docker. Run the following commands to build and run the Docker image:
```
$ docker build -t password_manager_web_app .
$ docker run -p 6966:6966 password_manager_web_app
```
