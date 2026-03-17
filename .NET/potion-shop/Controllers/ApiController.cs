using Microsoft.AspNetCore.Mvc;
using potion_shop.Models;
using potion_shop.Services;

namespace potion_shop.Controllers
{
    [ApiController]
    [Route("api")]
    public class ApiController : ControllerBase
    {
        private readonly ShopStore _shopStore;

        public ApiController(ShopStore shopStore)
        {
            _shopStore = shopStore;
        }

        /// <summary>
        /// Purchase a potion (POST /api/buy)
        /// </summary>
        [HttpPost("buy")]
        public async Task<IActionResult> Buy()
        {
            // Update shop statistics (Red potion costs 50 gold)
            var stats = _shopStore.IncrementSales(50);

            // Get a random fun message
            var message = Messages.GetRandomMessage();

            var response = new BuyResponse
            {
                Success = true,
                Message = message,
                Stats = stats
            };

            return await Task.FromResult(Ok(response));
        }

        /// <summary>
        /// Get shop statistics (GET /api/stats)
        /// </summary>
        [HttpGet("stats")]
        public async Task<IActionResult> GetStats()
        {
            var stats = _shopStore.GetStats();

            var response = new StatsResponse
            {
                PotionsSold = stats.PotionsSold,
                GoldEarned = stats.GoldEarned,
                GoldPerPotion = 50
            };

            return await Task.FromResult(Ok(response));
        }

        /// <summary>
        /// Reset shop statistics (POST /api/reset)
        /// </summary>
        [HttpPost("reset")]
        public async Task<IActionResult> Reset()
        {
            var stats = _shopStore.Reset();

            var response = new ResetResponse
            {
                Success = true,
                Message = "Shop statistics have been reset",
                Stats = stats
            };

            return await Task.FromResult(Ok(response));
        }

        /// <summary>
        /// Health check endpoint (GET /api/health)
        /// </summary>
        [HttpGet("health")]
        public async Task<IActionResult> Health()
        {
            var response = new HealthResponse
            {
                Status = "healthy",
                Shop = "open",
                PotionsAvailable = true
            };

            return await Task.FromResult(Ok(response));
        }
    }
}
