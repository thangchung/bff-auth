# Get starting

- Start backend

```bash
# Start auth-server and BFF server
> cd backend
> tye run
```

```bash
# Start sale-api
> cd backend\sale-api
> go run .
```

- Start frontend

```bash
# Start front-end
> cd frontend/bff-auth-nextjs
> yarn dev
```

- Go to `https://localhost:5002`, and start to play around

# High level architecture

![](assets/overview.png)

# Technical stacks

## BFF Authentication
- .NET 6

## Identity Server
- Duende.IdentityServer (.NET 6)

## Front-end
- Nextjs

## Product Api
- Rust (Axum): TODO

## Sale Api
- Golang (fiber)

# References
- RFC-8693: https://github.com/RockSolidKnowledge/TokenExchange
- https://datatracker.ietf.org/doc/html/draft-ietf-oauth-browser-based-apps-08
- https://github.com/manfredsteyer/yarp-auth-proxy
- https://developer.okta.com/blog/2021/01/04/offline-jwt-validation-with-go
