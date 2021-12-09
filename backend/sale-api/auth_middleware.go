package main

import (
	"crypto/rsa"
	"encoding/base64"
	"encoding/json"
	"errors"
	"math/big"
	"net/http"
	"os"
	"strings"

	"github.com/gofiber/fiber/v2"
	"github.com/golang-jwt/jwt"
)

var (
	rsakeys map[string]*rsa.PublicKey
	user_id string
)

func GetPublicKeys() {
	rsakeys = make(map[string]*rsa.PublicKey)
	var body map[string]interface{}
	uri := os.Getenv("AUTH_SERVER_KEY_URL")
	resp, _ := http.Get(uri)
	json.NewDecoder(resp.Body).Decode(&body)
	for _, bodykey := range body["keys"].([]interface{}) {
		key := bodykey.(map[string]interface{})
		kid := key["kid"].(string)
		rsakey := new(rsa.PublicKey)
		number, _ := base64.RawURLEncoding.DecodeString(key["n"].(string))
		rsakey.N = new(big.Int).SetBytes(number)
		rsakey.E = 65537
		rsakeys[kid] = rsakey
	}
}

func Verify(c *fiber.Ctx) bool {
	isValid := false
	errorMessage := ""
	tokenString := c.Get("Authorization")

	// we can get using fasthttp lib
	// v := c.Request().Header.Peek("Authorization")
	// tokenString := string(v)

	if strings.HasPrefix(tokenString, "Bearer ") {
		tokenString = strings.TrimPrefix(tokenString, "Bearer ")
		token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
			return rsakeys[token.Header["kid"].(string)], nil
		})
		if err != nil {
			errorMessage = err.Error()
		} else if !token.Valid {
			errorMessage = "Invalid token"
		} else if token.Header["alg"] == nil {
			errorMessage = "alg must be defined"
		} else {
			isValid = true
			user_id = token.Claims.(jwt.MapClaims)["sub"].(string)
		}
		if !isValid {
			c.SendStatus(http.StatusUnauthorized)
			c.SendString(errorMessage)
		}
	} else {
		c.SendStatus(http.StatusUnauthorized)
		c.SendString("Unauthorized")
	}
	return isValid
}

func JwtBearer(c *fiber.Ctx) error {
	GetPublicKeys()
	if !Verify(c) {
		return errors.New("invalid access")
	}

	return c.Next()
}
