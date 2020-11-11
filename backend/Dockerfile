# pull official base image
FROM node:lts-alpine

# set work directory
WORKDIR /srv/app/

# add to $PATH
ENV PATH /srv/app/node_modules/.bin:$PATH

# install app dependencies
COPY package.json ./
# COPY package-lock.json ./

# add app
COPY . ./

