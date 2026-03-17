package store

import (
	"math/rand"
	"sync"
	"time"

	"potion-shop-gin-k8s/internal/models"
)

// ShopStore manages the shop's in-memory statistics
type ShopStore struct {
	mu          sync.RWMutex
	potionsSold int
	goldEarned  int
}

// NewShopStore creates a new shop store
func NewShopStore() *ShopStore {
	return &ShopStore{
		potionsSold: 0,
		goldEarned:  0,
	}
}

// GetStats returns the current shop statistics
func (s *ShopStore) GetStats() models.ShopStats {
	s.mu.RLock()
	defer s.mu.RUnlock()

	return models.ShopStats{
		PotionsSold: s.potionsSold,
		GoldEarned:  s.goldEarned,
	}
}

// IncrementSales increments the shop statistics by the given gold amount
func (s *ShopStore) IncrementSales(goldAmount int) models.ShopStats {
	s.mu.Lock()
	defer s.mu.Unlock()

	s.potionsSold++
	s.goldEarned += goldAmount

	return models.ShopStats{
		PotionsSold: s.potionsSold,
		GoldEarned:  s.goldEarned,
	}
}

// Reset resets the shop statistics to zero
func (s *ShopStore) Reset() models.ShopStats {
	s.mu.Lock()
	defer s.mu.Unlock()

	s.potionsSold = 0
	s.goldEarned = 0

	return models.ShopStats{
		PotionsSold: s.potionsSold,
		GoldEarned:  s.goldEarned,
	}
}

// PurchaseMessages contains fun messages for potion purchases
var PurchaseMessages = []string{
	"You got a Red Potion! Your hearts are refilled!",
	"A fine purchase, brave hero! May it serve you well!",
	"One Red Potion coming right up! Stay safe out there!",
	"Excellent choice! This potion has saved many adventurers!",
	"A wise investment! You'll thank yourself later!",
	"Thank you for your patronage, noble warrior!",
	"It's dangerous to go alone! Take this potion!",
	"May this potion guide you to victory!",
}

// GetRandomMessage returns a random purchase message
func GetRandomMessage() string {
	rand.Seed(time.Now().UnixNano())
	return PurchaseMessages[rand.Intn(len(PurchaseMessages))]
}
