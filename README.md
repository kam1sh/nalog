# nalog
Nalog is a simple logging library, written in Rust.
The primary goal of nalog is to have impact on application performance as little as possible.

# Build from sources
## Requirements
The only requirements for nalog is Rust nightly-2019-07-19 and [maturin](https://github.com/PyO3/maturin).
If you new to rust, just install
```bash
rustup toolchain install nightly-2019-07-19
rustup override set nightly-2019-07-19
pip install maturin
```

## Build & install commands
```bash
maturin build -i python3.8 # 3.8 for example
pip install target/wheels/nalog-*.whl
```

# Usage
```python
from nalog import Logger
log = Logger(__name__, level="debug")
log.info("Hello!")
```
# Thanks
This library exists because of [PyO3](https://github.com/PyO3/pyo3).
Never thought that writing for Python in Rust can be so easy.
