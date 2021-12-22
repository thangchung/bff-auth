package middleware

import (
	"errors"
	"os"

	"github.com/gofiber/fiber/v2"
	"github.com/thangchung/bff-auth/backend/sale-api/gateway"
)

func PolicyEnforcer(c *fiber.Ctx) error {
	c.Set("content-type", "application/json")
	gateway := gateway.NewPolicyOpaGateway(os.Getenv("OPA_API_SERVER_URL"))

	if !gateway.Ask(c) {
		return errors.New("invalid access")
	}

	return c.Next()
}
