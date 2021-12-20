package http

default allow = true

allow = {
	"allow": true,
	"additional_headers": {"x-sub": my_claim},
} {
	my_claim := jwt.payload["sub"]
}

jwt = { "payload": payload } {
	auth_header := input.request.headers.Authorization
	[_, jwt] := split(auth_header, " ")
	[_, payload, _] := io.jwt.decode(jwt)
}
