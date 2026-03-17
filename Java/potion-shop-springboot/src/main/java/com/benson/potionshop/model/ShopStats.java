package com.benson.potionshop.model;

/**
 * Model class representing shop statistics
 */
public class ShopStats {
    private int potionsSold;
    private int goldEarned;

    public ShopStats() {
        this.potionsSold = 0;
        this.goldEarned = 0;
    }

    public ShopStats(int potionsSold, int goldEarned) {
        this.potionsSold = potionsSold;
        this.goldEarned = goldEarned;
    }

    public int getPotionsSold() {
        return potionsSold;
    }

    public void setPotionsSold(int potionsSold) {
        this.potionsSold = potionsSold;
    }

    public int getGoldEarned() {
        return goldEarned;
    }

    public void setGoldEarned(int goldEarned) {
        this.goldEarned = goldEarned;
    }

    public void incrementSales(int goldAmount) {
        this.potionsSold++;
        this.goldEarned += goldAmount;
    }

    public void reset() {
        this.potionsSold = 0;
        this.goldEarned = 0;
    }
}
