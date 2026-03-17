package com.benson.potionshop.service;

import com.benson.potionshop.model.ShopStats;
import org.springframework.stereotype.Service;

@Service
public class ShopService {

    private static final int RED_POTION_PRICE = 50;
    private final ShopStats stats = new ShopStats();

    public ShopStats getStats() {
        return stats.copy();
    }

    public ShopStats purchasePotion() {
        stats.incrementSales(RED_POTION_PRICE);
        return stats.copy();
    }

    public ShopStats resetStats() {
        stats.reset();
        return stats.copy();
    }

    public int getPotionPrice() {
        return RED_POTION_PRICE;
    }
}
