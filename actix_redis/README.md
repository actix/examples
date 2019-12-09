# Actix Web Redis ASync Writer

## Run

- make sure docker with docker-compose and is installed

- run ``` docker-compose up ``` from the command line

- then open browser to [localhost:3000](http://localhost:3000) that will insert records into redis

- then open browser to [localhost:5001](http://localhost:5001) and type in {hostname: redis, port: 6379, database_id: 0} for the fields for rebrow then click on Keys to view entries
