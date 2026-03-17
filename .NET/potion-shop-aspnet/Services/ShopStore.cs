using potion_shop.Models;

namespace potion_shop.Services
{
    /// <summary>
    /// Simple in-memory storage for shop statistics
    /// In a production app, you'd want to use a database
    /// </summary>
    public class ShopStore
    {
        private ShopStats _stats = new ShopStats
        {
            PotionsSold = 0,
            GoldEarned = 0
        };

        private readonly object _lock = new object();

        public ShopStats GetStats()
        {
            lock (_lock)
            {
                return new ShopStats
                {
                    PotionsSold = _stats.PotionsSold,
                    GoldEarned = _stats.GoldEarned
                };
            }
        }

        public ShopStats IncrementSales(int goldAmount)
        {
            lock (_lock)
            {
                _stats.PotionsSold += 1;
                _stats.GoldEarned += goldAmount;
                return new ShopStats
                {
                    PotionsSold = _stats.PotionsSold,
                    GoldEarned = _stats.GoldEarned
                };
            }
        }

        public ShopStats Reset()
        {
            lock (_lock)
            {
                _stats.PotionsSold = 0;
                _stats.GoldEarned = 0;
                return new ShopStats
                {
                    PotionsSold = _stats.PotionsSold,
                    GoldEarned = _stats.GoldEarned
                };
            }
        }
    }
}
