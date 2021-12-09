# Get starting

- Start backend

```bash
> cd backend
> tye run
```

- Start frontend

```bash
> cd frontend/bff-auth-nextjs
> yarn dev
```

- Go to `http://localhost:3000`, and start to play around

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
- https://github.com/manfredsteyer/yarp-auth-proxy
- https://developer.okta.com/blog/2021/01/04/offline-jwt-validation-with-go
