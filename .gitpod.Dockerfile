FROM gitpod/workspace-full:latest

# Install system-tools
# RUN brew install fish curl git nodenv
RUN sudo apt-get update
RUN sudo apt-get install -y fish curl git bash perl
# RUN sudo rm -rf /var/lib/apt/lists/*
RUN brew update

# Install Node
RUN brew install nodenv &> /dev/null
RUN eval "$(nodenv init -)"
RUN nodenv install 16.10.0
RUN node --version
RUN npm --version
RUN npm install
RUN npm run build:dev

# Install python
RUN brew install pre-commit
# RUN pre-commit install
# RUN pre-commit run --all-files

# Install Rust
RUN cargo install cargo-edit
RUN cargo install cargo-insta
RUN cargo install bunyan
RUN cargo --version
RUN cargo build
