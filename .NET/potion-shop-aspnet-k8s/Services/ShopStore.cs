using potion_shop_k8s.Models;

namespace potion_shop_k8s.Services
{
    /// <summary>
    /// Simple in-memory storage for shop statistics
    /// In a production app, you'd want to use a database
    /// </summary>
    public class ShopStore
    {
        private readonly object _lock = new object();
        private int _potionsSold = 0;
        private int _goldEarned = 0;

        /// <summary>
        /// Get current shop statistics
        /// </summary>
        public ShopStats GetStats()
        {
            lock (_lock)
            {
                return new ShopStats
                {
                    PotionsSold = _potionsSold,
                    GoldEarned = _goldEarned
                };
            }
        }

        /// <summary>
        /// Increment sales statistics
        /// </summary>
        /// <param name="goldAmount">Amount of gold from the sale</param>
        public ShopStats IncrementSales(int goldAmount)
        {
            lock (_lock)
            {
                _potionsSold++;
                _goldEarned += goldAmount;
                return new ShopStats
                {
                    PotionsSold = _potionsSold,
                    GoldEarned = _goldEarned
                };
            }
        }

        /// <summary>
        /// Reset shop statistics
        /// </summary>
        public ShopStats Reset()
        {
            lock (_lock)
            {
                _potionsSold = 0;
                _goldEarned = 0;
                return new ShopStats
                {
                    PotionsSold = _potionsSold,
                    GoldEarned = _goldEarned
                };
            }
        }
    }
}
