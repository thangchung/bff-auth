FROM golang:1.17.3-buster AS build
WORKDIR /src/

COPY go.mod go.sum ./
RUN go mod download

COPY *.go ./

RUN CGO_ENABLED=0 go build -o /bin/main

FROM scratch
COPY --from=build /bin/main /bin/main
EXPOSE 5003
ENTRYPOINT ["/bin/main"]