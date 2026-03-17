package main

import (
	"log"
	"os"

	"github.com/gin-gonic/gin"
	"potion-shop-gin-k8s/internal/handlers"
	"potion-shop-gin-k8s/internal/store"
)

func main() {
	// Set Gin to release mode in production
	if os.Getenv("GIN_MODE") == "" {
		gin.SetMode(gin.ReleaseMode)
	}

	// Create router
	router := gin.Default()

	// Load HTML templates
	router.LoadHTMLGlob("templates/*")

	// Initialize shop store
	shopStore := store.NewShopStore()

	// Initialize handlers
	handler := handlers.NewHandler(shopStore)

	// Routes
	router.GET("/", handler.GetIndex)
	router.POST("/api/buy", handler.BuyPotion)
	router.GET("/api/stats", handler.GetStats)
	router.POST("/api/reset", handler.ResetStats)
	router.GET("/api/health", handler.HealthCheck)

	// Get port from environment or use default
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	// Start server
	log.Printf("Starting Benson's Potion Store on port %s...", port)
	if err := router.Run(":" + port); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
