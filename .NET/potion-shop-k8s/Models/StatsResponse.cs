namespace potion_shop_k8s.Models
{
    /// <summary>
    /// Response model for stats endpoint
    /// </summary>
    public class StatsResponse
    {
        public int PotionsSold { get; set; }
        public int GoldEarned { get; set; }
        public int GoldPerPotion { get; set; } = 50;
    }
}
