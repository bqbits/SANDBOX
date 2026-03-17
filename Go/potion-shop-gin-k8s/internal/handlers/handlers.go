package handlers

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"potion-shop-gin-k8s/internal/models"
	"potion-shop-gin-k8s/internal/store"
)

// Handler holds the shop store
type Handler struct {
	store *store.ShopStore
}

// NewHandler creates a new handler
func NewHandler(s *store.ShopStore) *Handler {
	return &Handler{
		store: s,
	}
}

// GetIndex serves the main shop HTML page
func (h *Handler) GetIndex(c *gin.Context) {
	c.HTML(http.StatusOK, "index.html", nil)
}

// BuyPotion handles potion purchases
func (h *Handler) BuyPotion(c *gin.Context) {
	// Red potion costs 50 gold
	const potionPrice = 50

	// Update shop statistics
	stats := h.store.IncrementSales(potionPrice)

	// Get a random fun message
	message := store.GetRandomMessage()

	// Return response
	c.JSON(http.StatusOK, models.BuyResponse{
		Success: true,
		Message: message,
		Stats:   stats,
	})
}

// GetStats returns the current shop statistics
func (h *Handler) GetStats(c *gin.Context) {
	stats := h.store.GetStats()
	c.JSON(http.StatusOK, stats)
}

// ResetStats resets the shop statistics
func (h *Handler) ResetStats(c *gin.Context) {
	stats := h.store.Reset()
	c.JSON(http.StatusOK, models.ResetResponse{
		Success: true,
		Stats:   stats,
	})
}

// HealthCheck returns the health status
func (h *Handler) HealthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, models.HealthResponse{
		Status: "healthy",
		Time:   time.Now().Format(time.RFC3339),
	})
}
