package models

// ShopStats represents the shop's statistics
type ShopStats struct {
	PotionsSold int `json:"potions_sold"`
	GoldEarned  int `json:"gold_earned"`
}

// BuyResponse represents the response when buying a potion
type BuyResponse struct {
	Success bool      `json:"success"`
	Message string    `json:"message"`
	Stats   ShopStats `json:"stats"`
}

// ResetResponse represents the response when resetting stats
type ResetResponse struct {
	Success bool      `json:"success"`
	Stats   ShopStats `json:"stats"`
}

// HealthResponse represents the health check response
type HealthResponse struct {
	Status string `json:"status"`
}
