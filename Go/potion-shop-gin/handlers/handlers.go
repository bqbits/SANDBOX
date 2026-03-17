package handlers

import (
	"net/http"
	"potion-shop-gin/models"
	"potion-shop-gin/store"

	"github.com/gin-gonic/gin"
)

// Handler holds references to dependencies
type Handler struct {
	store *store.ShopStore
}

// NewHandler creates a new handler instance
func NewHandler(s *store.ShopStore) *Handler {
	return &Handler{
		store: s,
	}
}

// GetStats returns the current shop statistics
func (h *Handler) GetStats(c *gin.Context) {
	stats := h.store.GetStats()
	c.JSON(http.StatusOK, stats)
}

// BuyPotion handles purchasing a potion
func (h *Handler) BuyPotion(c *gin.Context) {
	const redPotionPrice = 50

	// Update shop statistics
	stats := h.store.IncrementSales(redPotionPrice)

	// Get a random fun message
	message := h.store.GetRandomMessage()

	response := models.BuyResponse{
		Success: true,
		Message: message,
		Stats:   stats,
	}

	c.JSON(http.StatusOK, response)
}

// ResetStats resets the shop statistics
func (h *Handler) ResetStats(c *gin.Context) {
	stats := h.store.Reset()

	response := models.ResetResponse{
		Success: true,
		Stats:   stats,
	}

	c.JSON(http.StatusOK, response)
}

// HealthCheck returns the health status
func (h *Handler) HealthCheck(c *gin.Context) {
	response := models.HealthResponse{
		Status: "ok",
	}
	c.JSON(http.StatusOK, response)
}
