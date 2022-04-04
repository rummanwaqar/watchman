 docker run \
    -p "6379:6379" \
    -d \
    --name "redis_$(date '+%s')" \
    redis:6