package http

default allow = false

allow {
	some userid
	input.method == "GET"
  	input.path = ["api", userid]
  	jwt.payload.sub == userid
	jwt.payload.sub == "88421113"
}

jwt = { "payload": payload } {
	auth_header := input.token
	[_, jwt] := split(auth_header, " ")
	[_, payload, _] := io.jwt.decode(jwt)
}
