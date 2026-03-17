using Microsoft.AspNetCore.Mvc;
using potion_shop_k8s.Models;
using potion_shop_k8s.Services;

namespace potion_shop_k8s.Controllers
{
    /// <summary>
    /// API endpoints for the potion shop
    /// </summary>
    [ApiController]
    [Route("api")]
    public class ApiController : ControllerBase
    {
        private readonly ShopStore _shopStore;
        private readonly MessageService _messageService;
        private readonly ILogger<ApiController> _logger;

        public ApiController(
            ShopStore shopStore,
            MessageService messageService,
            ILogger<ApiController> logger)
        {
            _shopStore = shopStore;
            _messageService = messageService;
            _logger = logger;
        }

        /// <summary>
        /// Health check endpoint for Kubernetes probes
        /// </summary>
        [HttpGet("health")]
        public IActionResult Health()
        {
            return Ok(new HealthResponse
            {
                Status = "healthy",
                Shop = "open",
                PotionsAvailable = true
            });
        }

        /// <summary>
        /// Get shop statistics
        /// </summary>
        [HttpGet("stats")]
        public IActionResult GetStats()
        {
            var stats = _shopStore.GetStats();
            return Ok(new StatsResponse
            {
                PotionsSold = stats.PotionsSold,
                GoldEarned = stats.GoldEarned,
                GoldPerPotion = 50
            });
        }

        /// <summary>
        /// Purchase a potion
        /// </summary>
        [HttpPost("buy")]
        public IActionResult BuyPotion()
        {
            // Red potion costs 50 gold
            var stats = _shopStore.IncrementSales(50);
            var message = _messageService.GetRandomMessage();

            _logger.LogInformation("Potion purchased! Total sold: {PotionsSold}, Gold earned: {GoldEarned}",
                stats.PotionsSold, stats.GoldEarned);

            return Ok(new BuyResponse
            {
                Success = true,
                Message = message,
                Stats = stats
            });
        }

        /// <summary>
        /// Reset shop statistics
        /// </summary>
        [HttpPost("reset")]
        public IActionResult ResetStats()
        {
            var stats = _shopStore.Reset();

            _logger.LogInformation("Shop statistics reset");

            return Ok(new
            {
                success = true,
                message = "Shop statistics have been reset",
                stats = stats
            });
        }
    }
}
