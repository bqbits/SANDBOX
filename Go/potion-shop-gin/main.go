package main

import (
	"log"
	"potion-shop-gin/handlers"
	"potion-shop-gin/store"

	"github.com/gin-gonic/gin"
)

func main() {
	// Create shop store
	shopStore := store.NewShopStore()

	// Create handlers
	handler := handlers.NewHandler(shopStore)

	// Set Gin to release mode for production
	// Comment out the next line for development/debugging
	// gin.SetMode(gin.ReleaseMode)

	// Create Gin router
	router := gin.Default()

	// Serve static files (HTML, CSS, JS)
	router.StaticFile("/", "./static/index.html")
	router.Static("/static", "./static")

	// API routes
	api := router.Group("/api")
	{
		api.GET("/stats", handler.GetStats)
		api.POST("/buy", handler.BuyPotion)
		api.POST("/reset", handler.ResetStats)
		api.GET("/health", handler.HealthCheck)
	}

	// Start server
	port := ":8080"
	log.Printf("🏺 Starting Benson's Potion Shop on http://localhost%s", port)
	log.Printf("⚔️ Press Ctrl+C to stop the server")

	if err := router.Run(port); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
