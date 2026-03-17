namespace potion_shop.Models
{
    public class ShopStats
    {
        public int PotionsSold { get; set; }
        public int GoldEarned { get; set; }
    }

    public class BuyResponse
    {
        public bool Success { get; set; }
        public string Message { get; set; } = string.Empty;
        public ShopStats Stats { get; set; } = new ShopStats();
    }

    public class StatsResponse
    {
        public int PotionsSold { get; set; }
        public int GoldEarned { get; set; }
        public int GoldPerPotion { get; set; }
    }

    public class ResetResponse
    {
        public bool Success { get; set; }
        public string Message { get; set; } = string.Empty;
        public ShopStats Stats { get; set; } = new ShopStats();
    }

    public class HealthResponse
    {
        public string Status { get; set; } = string.Empty;
        public string Shop { get; set; } = string.Empty;
        public bool PotionsAvailable { get; set; }
    }
}
