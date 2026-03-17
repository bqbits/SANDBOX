/**
 * Simple in-memory storage for shop statistics
 * In a production app, you'd want to use a database
 */

export interface ShopStats {
  potions_sold: number;
  gold_earned: number;
}

// In-memory store
const shopStats: ShopStats = {
  potions_sold: 0,
  gold_earned: 0,
};

export const shopStore = {
  getStats: (): ShopStats => {
    return { ...shopStats };
  },

  incrementSales: (goldAmount: number): ShopStats => {
    shopStats.potions_sold += 1;
    shopStats.gold_earned += goldAmount;
    return { ...shopStats };
  },

  reset: (): ShopStats => {
    shopStats.potions_sold = 0;
    shopStats.gold_earned = 0;
    return { ...shopStats };
  },
};
