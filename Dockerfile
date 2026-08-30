FROM scratch

COPY target/x86_64-unknown-linux-gnu/release/health_http /bin/health-api

ENTRYPOINT ["/bin/health-api"]
