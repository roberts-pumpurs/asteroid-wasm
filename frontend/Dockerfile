#  ----------- NODE  --------- #
FROM node:lts-buster

# set work directory
WORKDIR /srv/app/

# add to $PATH
ENV PATH /srv/app/node_modules/.bin:$PATH

# install app dependencies
COPY package.json ./
RUN npm install react-scripts@3.4.1 -g

# ------------- RUST ------------- #
RUN curl --proto '=https' --tlsv1.2  -sSf https://sh.rustup.rs  | bash -s -- -y

# add to $PATH
ENV PATH="/root/.cargo/bin:${PATH}"


RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | bash
RUN cargo install cargo-watch

# For deploytment
RUN apt update
RUN apt install xsel

# add app
COPY . ./

