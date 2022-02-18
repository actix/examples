# protobuf

## Usage

### Server

```shell
cd protobuf
cargo run
```

### Client

```shell
# Dependencies
wget https://github.com/protocolbuffers/protobuf/releases/download/v3.11.2/protobuf-python-3.11.2.zip
unzip protobuf-python-3.11.2.zip
cd protobuf-3.11.2/python/
python3 setup.py install
pip3 install --upgrade pip
pip3 install aiohttp

# Client
cd ../..
python3 client.py
```
