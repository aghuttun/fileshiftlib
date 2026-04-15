# fileshiftlib

SFTP client for Python.

- [Installation](#installation)
- [Usage](#usage)
- [Version](#version)
- [License](#license)

## Installation

```bash
pip install fileshiftlib
```

## Usage

```python
import fileshiftlib

# Initialise SFTP client
sftp = fileshiftlib.SFTP(
    host="sftp.example.com",
    username="user",
    password="password",
    port=22
)
```

```python
# Check connection status
if sftp.is_connected():
    print("Connected!")
```

```python
# List directory contents
files = sftp.list_dir(".")
print(files)
```

```python
# Change directory
sftp.change_dir("/remote/path")
```

```python
# Upload a file
sftp.upload_file("local_file.txt", "remote_file.txt")
```

```python
# Download a file
sftp.download_file("remote_file.txt", "local_copy.txt")
```

```python
# Delete a file
sftp.delete_file("remote_file.txt")
```

```python
# Reconnect
sftp.reconnect()
```

## Version

Recommended way to read the installed package version:

```python
from importlib.metadata import version

print(version("fileshiftlib"))
```

Convenience attribute (also available):

```python
import fileshiftlib

print(fileshiftlib.__version__)
```

## License

BSD-3-Clause License (see [LICENSE](LICENSE))
