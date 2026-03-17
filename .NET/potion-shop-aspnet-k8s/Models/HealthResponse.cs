namespace potion_shop_k8s.Models
{
    /// <summary>
    /// Response model for health check endpoint
    /// </summary>
    public class HealthResponse
    {
        public string Status { get; set; } = "healthy";
        public string Shop { get; set; } = "open";
        public bool PotionsAvailable { get; set; } = true;
    }
}
