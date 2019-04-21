import requests

print('ping: pong immediately')
r = requests.post('http://127.0.0.1:8080/', json={
    'jsonrpc': '2.0',
    'method': 'ping',
    'params': [],
    'id': 1
})
print(r.json())


print('ping: pong after 4 secs')
r = requests.post('http://127.0.0.1:8080/', json={
    'jsonrpc': '2.0',
    'method': 'wait',
    'params': [4],
    'id': 1
})
print(r.json())

for i in range(10):
    print(f'inc {i:>02}')
    r = requests.post('http://127.0.0.1:8080/', json={
        'jsonrpc': '2.0',
        'method': 'inc',
        'params': [],
        'id': 1
    })

print(f'get')
r = requests.post('http://127.0.0.1:8080/', json={
    'jsonrpc': '2.0',
    'method': 'get',
    'params': [],
    'id': 1
})
print(r.json())
