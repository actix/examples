
Build image:

```shell
  docker build -t docker_sample --force-rm --no-cache -f  Dockerfile .
```

Run image:

```shell
  
  echo "== start sample_docker"
  docker run -d -p 5000:5000 docker_sample &
  docker ps

  echo "== wait 3s for startup"
  sleep 3s

  echo "== curl both routes"
  curl http://localhost:5000
  curl http://localhost:5000/again 

```