# FROM gcr.io/distroless/base-debian12
FROM --platform=linux/amd64 debian:bullseye-slim
# Use jemalloc as memory allocator 
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates
ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so
# Both bench and release profiles copy from release dir
COPY sui-proxy /opt/sui/bin/sui-proxy
COPY sui-proxy.yaml /opt/sui/config/sui-proxy.yaml

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION