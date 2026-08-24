FROM scratch

COPY target/release/health_http /bin/health-api

ENTRYPOINT ["/bin/health-api"]
