<?php

namespace Tests\Feature;

use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class ShopTest extends TestCase
{
    /**
     * Test that the shop page loads successfully.
     */
    public function test_shop_page_loads(): void
    {
        $response = $this->get('/');

        $response->assertStatus(200);
        $response->assertSee('Benson\'s Potion Store');
        $response->assertSee('RED POTION');
    }

    /**
     * Test buying a potion.
     */
    public function test_can_buy_potion(): void
    {
        $response = $this->post('/api/buy');

        $response->assertStatus(200);
        $response->assertJson([
            'success' => true,
        ]);
        $response->assertJsonStructure([
            'success',
            'message',
            'stats' => [
                'potions_sold',
                'gold_earned',
            ],
        ]);
    }

    /**
     * Test getting stats.
     */
    public function test_can_get_stats(): void
    {
        $response = $this->get('/api/stats');

        $response->assertStatus(200);
        $response->assertJsonStructure([
            'potions_sold',
            'gold_earned',
        ]);
    }

    /**
     * Test resetting stats.
     */
    public function test_can_reset_stats(): void
    {
        // Buy a potion first
        $this->post('/api/buy');

        // Reset stats
        $response = $this->post('/api/reset');

        $response->assertStatus(200);
        $response->assertJson([
            'success' => true,
            'stats' => [
                'potions_sold' => 0,
                'gold_earned' => 0,
            ],
        ]);
    }

    /**
     * Test health check endpoint.
     */
    public function test_health_check(): void
    {
        $response = $this->get('/api/health');

        $response->assertStatus(200);
        $response->assertJson([
            'status' => 'ok',
        ]);
    }

    /**
     * Test that buying multiple potions increments stats correctly.
     */
    public function test_buying_multiple_potions_increments_stats(): void
    {
        // Buy 3 potions
        $this->post('/api/buy');
        $this->post('/api/buy');
        $response = $this->post('/api/buy');

        $response->assertStatus(200);
        $response->assertJson([
            'success' => true,
            'stats' => [
                'potions_sold' => 3,
                'gold_earned' => 150,
            ],
        ]);
    }
}
