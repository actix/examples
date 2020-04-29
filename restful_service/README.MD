
### 世间唯你好, 余者皆枯骨
```
    Rust写的一个Restful接口微服务, 尽量符合springboot的风格, 使用middleware处理aop, 全局异常处理器等.
    - actix
    - actix_web
    - deadpool
    - tokio-pg
```

#### GET( within query parameters)

- GET: http://localhost:8080/api/region/simple/cities?pid=36

    ```
    {
        "code": 0,
        "message": "success",
        "data": [
            {
                "id": 37,
                "name": "石家庄市",
                "pid": 36,
                "province": "河北省",
                "lgt": 114.52,
                "lat": 38.05
            },
            {
                "id": 59,
                "name": "唐山市",
                "pid": 36,
                "province": "河北省",
                "lgt": 118.2,
                "lat": 39.63
            },
            {
                "id": 75,
                "name": "秦皇岛市",
                "pid": 36,
                "province": "河北省",
                "lgt": 119.6,
                "lat": 39.93
            },
            {
                "id": 83,
                "name": "邯郸市",
                "pid": 36,
                "province": "河北省",
                "lgt": 114.48,
                "lat": 36.62
            },
            {
                "id": 103,
                "name": "邢台市",
                "pid": 36,
                "province": "河北省",
                "lgt": 114.48,
                "lat": 37.07
            },
            {
                "id": 123,
                "name": "保定市",
                "pid": 36,
                "province": "河北省",
                "lgt": 115.47,
                "lat": 38.87
            },
            {
                "id": 148,
                "name": "张家口市",
                "pid": 36,
                "province": "河北省",
                "lgt": 114.88,
                "lat": 40.82
            },
            {
                "id": 166,
                "name": "承德市",
                "pid": 36,
                "province": "河北省",
                "lgt": 117.93,
                "lat": 40.97
            },
            {
                "id": 178,
                "name": "沧州市",
                "pid": 36,
                "province": "河北省",
                "lgt": 116.83,
                "lat": 38.3
            },
            {
                "id": 195,
                "name": "廊坊市",
                "pid": 36,
                "province": "河北省",
                "lgt": 116.7,
                "lat": 39.52
            },
            {
                "id": 206,
                "name": "衡水市",
                "pid": 36,
                "province": "河北省",
                "lgt": 115.68,
                "lat": 37.73
            }
        ]
    }

    ```

- GET: http://localhost:8080/api/region/simple/cities

    ```
    {
        "code": -1,
        "message": "parameters are illegal"
    }
    ```

- GET: http://localhost:8080/api/region/simple/cities?pid=abc

    ```
    {
        "code": -1,
        "message": "cant convert paramter into number"
    }
    ```