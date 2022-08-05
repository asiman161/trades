FROM golang:1.18-alpine as build

WORKDIR /back

COPY . /back

RUN go mod tidy

RUN go build cmd/main.go

CMD ["./main"]
