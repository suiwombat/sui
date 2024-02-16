FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
# RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates
# ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so
# Both bench and release profiles copy from release dir
COPY hello_linux_x86_64 /opt/sui/bin/sui-proxy
# Support legacy usages of /usr/local/bin/sui-node
RUN ln -s /opt/sui/bin/sui-proxy /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
