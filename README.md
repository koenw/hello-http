# hello-http

Dockerized HTTP hello-world, for when you just need a placeholder to respond to
HTTP requests.

## Building the docker image

`docker build -t hello-http:0.1.0 .`

## Configuration

You can specify the port to listen on by setting the `HTTP_PORT` environmental
variable (defaults to 8000).
