package store

import (
	"math/rand"
	"potion-shop-gin/models"
	"sync"
	"time"
)

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

// ShopStore manages the shop's statistics
type ShopStore struct {
	stats models.ShopStats
	mu    sync.RWMutex
	rand  *rand.Rand
}

// NewShopStore creates a new shop store instance
func NewShopStore() *ShopStore {
	return &ShopStore{
		stats: models.ShopStats{
			PotionsSold: 0,
			GoldEarned:  0,
		},
		rand: rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

// GetStats returns the current shop statistics
func (s *ShopStore) GetStats() models.ShopStats {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return models.ShopStats{
		PotionsSold: s.stats.PotionsSold,
		GoldEarned:  s.stats.GoldEarned,
	}
}

// IncrementSales increments potions sold and gold earned
func (s *ShopStore) IncrementSales(goldAmount int) models.ShopStats {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.stats.PotionsSold++
	s.stats.GoldEarned += goldAmount
	return models.ShopStats{
		PotionsSold: s.stats.PotionsSold,
		GoldEarned:  s.stats.GoldEarned,
	}
}

// Reset resets the shop statistics to zero
func (s *ShopStore) Reset() models.ShopStats {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.stats.PotionsSold = 0
	s.stats.GoldEarned = 0
	return models.ShopStats{
		PotionsSold: s.stats.PotionsSold,
		GoldEarned:  s.stats.GoldEarned,
	}
}

// GetRandomMessage returns a random purchase message
func (s *ShopStore) GetRandomMessage() string {
	s.mu.Lock()
	defer s.mu.Unlock()
	return PurchaseMessages[s.rand.Intn(len(PurchaseMessages))]
}
