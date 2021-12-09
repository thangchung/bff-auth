package main

import (
	"fmt"
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/joho/godotenv"
)

func main() {
	err := godotenv.Load()

	if err != nil {
		fmt.Print("Error loading .env file")
	}

	app := fiber.New()

	app.Use(JwtBearer)

	app.Get("/api/:name", func(c *fiber.Ctx) error {
		msg := fmt.Sprintf("Hello, %s 👋! from user_id %s", c.Params("name"), user_id)
		return c.SendString(msg)
	})

	log.Fatal(app.Listen(":5004"))
}
