package main

import (
	"fmt"
	"log"

	swagger "github.com/arsmn/fiber-swagger/v2"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/joho/godotenv"

	_ "github.com/thangchung/bff-auth/backend/sale-api/docs"
)

// @title Sale APIs
// @version 1.0
// @description This is a Sale APIs swagger
// @termsOfService http://swagger.io/terms/
// @contact.name API Support
// @contact.email fiber@swagger.io
// @license.name MIT
// @license.url https://opensource.org/licenses/MIT
// @host localhost:5004
// @BasePath /
func main() {
	if err := godotenv.Load(); err != nil {
		log.Println("warning .env file load error", err)
	}

	app := fiber.New()

	app.Use(cors.New())
	app.Use(logger.New())
	app.Use(JwtBearer)

	app.Get("/swagger/*", swagger.Handler) // default

	app.Get("/", func(c *fiber.Ctx) error {
		return c.Redirect("/swagger/")
	})

	app.Get("/api/:name", Hello)

	log.Fatal(app.Listen(":5004"))
}

// Hello godoc
// @Summary      Say hello
// @Description  get hello
// @Accept       json
// @Produce      json
// @Param name 	 path string true "Name"
// @Success 200 {object} string
// @Failure 400 {object} HTTPError
// @Failure 404 {object} HTTPError
// @Failure 500 {object} HTTPError
// @Router       /api/{name} [get]
func Hello(c *fiber.Ctx) error {
	msg := fmt.Sprintf("Hello, %s 👋! from user_id %s", c.Params("name"), user_id)
	return c.SendString(msg)
}

type HTTPError struct {
	status  string
	message string
}
