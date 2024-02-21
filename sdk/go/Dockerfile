FROM golang:1.22.0-alpine3.19 AS go-builder

WORKDIR /app
COPY go.mod .
COPY go.sum .
RUN go mod download

COPY . .
RUN go build \
    -o random_actor \
    -ldflags="-s -w" \
    -trimpath \
    ./cmd/random_actor

FROM alpine:3.19 as runner

RUN apk add --no-cache curl

FROM runner

WORKDIR /app
COPY --from=go-builder /app/random_actor /app/random_actor
COPY wait-for-server.sh /app/wait-for-server.sh
RUN chmod +x /app/wait-for-server.sh

ENV CLIENT_SECRET=/app/secrets/secret
ENTRYPOINT ["/app/wait-for-server.sh", "/app/random_actor"]

