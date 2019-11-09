FROM alpine:3.10
#RUN apk add libseccomp
COPY target/x86_64-unknown-linux-musl/release/rustimate .
EXPOSE 5500/tcp
ENTRYPOINT ["/rustimate"]
