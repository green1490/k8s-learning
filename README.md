webserver:
    - rust
    - contain test cases
    - contain env variable for connection
    - simple login and registering
    - login gives back a login/session token

CI/CD:
    - doesnt allow to push into the master
    - doesnt allow push in the master if the build or test cases fails
    - checks for token
    - build image and push into the registry
    - format files
    - check for license compliance
    - check for application vulnerability

database:
    - postgres
    - on startup tables are created

monitoring:
    - monitor the state of the cluster

k8s:
    - blue/green deploy
    - kind for integration test

Requisition:
    - creating k8s cluster with kind
    - deploying the yml files