package main

import "blinkhost.dev/sdk/go/blinkhost"

func main() {
	response, err := blinkhost.JSONResponse(200, map[string]any{
		"language": "go",
		"canary":   true,
	})
	if err != nil || blinkhost.Respond(response) != nil {
		panic("response failed")
	}
}
