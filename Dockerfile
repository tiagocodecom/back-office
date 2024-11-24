ARG ALPINE_VERSION=latest

FROM alpine:${ALPINE_VERSION}

LABEL author="Tiagocode <santiagomm1997@gmail.com>"
LABEL mantainer="Tiagocode <santiagomm1997@gmail.com>"

WORKDIR /var/www

COPY . /var/www
