# Base on the most recently released Fedora
FROM gcr.io/distroless/cc:latest
# FROM ubuntu:22.04
MAINTAINER suiwombat email noreply@mystenlabs.com

# ARG SUI_PROXY_BUILD
# COPY $SUI_PROXY_BUILD sui-proxy
COPY sui_proxy /sui-proxy
COPY src/data/config.yaml /sui-proxy.yaml
COPY fullchain.pem /fullchain.pem
COPY privkey.pem /privkey.pem
EXPOSE 8080
EXPOSE 9184

CMD ["/sui-proxy"]