# Why BFF for Authentication?

TODO

# When to use BFF Auth with Cookies-based
- Used: 
  - Cookies (Same site) and move the authentication to the trusted backend
  - Use a dedicated backend for SPA now and move the security to the trusted backend
  - Azure does not support introspection or the revocation endpoint so you cannot invalidate the tokens, or logout an Azure SPA fully. BBF removes this problem
- Not used:
  - High load apps or cross domain with high load
  - In this case, please use tokens, but it has a risk of tokens to be robbed in the client-side

# Get starting

- Start frontend

```bash
# Start front-end
> cd frontend/bff-auth-nextjs
> yarn dev
```

- Start backend

```bash
# Start auth-server, BFF server, and sale-api
> tye run
```

- Go to `https://localhost:8080`, and start to play with it

![](assets/auth_flow.gif)

# High level architecture

![](assets/overview.png)

# Hosts and Services

<table>
  <tr>
    <td>No.</td>
    <td>Name</td>
    <td>URI</td>
  </tr>
  <tr>
    <td>1</td>
    <td>Gateway (BFF Auth)</td>
    <td>https://localhost:8080</td>
  </tr>
  <tr>
    <td>2</td>
    <td>Duende.IdentityServer</td>
    <td>https://localhost:5001</td>
  </tr>
  <tr>
    <td>3</td>
    <td>Product API (TODO)</td>
    <td>http://localhost:5003</td>
  </tr>
  <tr>
    <td>4</td>
    <td>Sale API</td>
    <td>http://localhost:5004</td>
  </tr>
  <tr>
    <td>5</td>
    <td>Ship API (TODO)</td>
    <td>http://localhost:5005</td>
  </tr>
  <tr>
    <td>5</td>
    <td>Web (Nextjs)</td>
    <td>http://localhost:3000</td>
  </tr>
</table>

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

## Ship Api
- .NET 6

# References
- RFC-8693: https://github.com/RockSolidKnowledge/TokenExchange
- https://datatracker.ietf.org/doc/html/draft-ietf-oauth-browser-based-apps-08
- https://github.com/manfredsteyer/yarp-auth-proxy
- https://developer.okta.com/blog/2021/01/04/offline-jwt-validation-with-go
