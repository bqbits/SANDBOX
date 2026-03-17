namespace potion_shop_k8s.Models
{
    /// <summary>
    /// Response model for buy endpoint
    /// </summary>
    public class BuyResponse
    {
        public bool Success { get; set; }
        public string Message { get; set; } = string.Empty;
        public ShopStats Stats { get; set; } = new ShopStats();
    }
}
