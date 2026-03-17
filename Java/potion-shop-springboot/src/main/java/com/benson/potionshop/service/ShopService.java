package com.benson.potionshop.service;

import com.benson.potionshop.model.ShopStats;
import org.springframework.stereotype.Service;

/**
 * Service for managing shop statistics
 * Uses in-memory storage (no database)
 */
@Service
public class ShopService {

    private final ShopStats stats = new ShopStats();

    public ShopStats getStats() {
        return new ShopStats(stats.getPotionsSold(), stats.getGoldEarned());
    }

    public ShopStats incrementSales(int goldAmount) {
        stats.incrementSales(goldAmount);
        return getStats();
    }

    public ShopStats reset() {
        stats.reset();
        return getStats();
    }
}
