#!/bin/bash

docker stop $(docker -aq)
docker rm $(docker -aq)

docker run -d --restart=always -v /root/l.json:/app/l.json axelharold/ore:v1000  --priority-fee 1000 mine --threads 8