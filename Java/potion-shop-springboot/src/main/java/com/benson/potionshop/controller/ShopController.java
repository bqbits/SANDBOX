package com.benson.potionshop.controller;

import com.benson.potionshop.model.ShopStats;
import com.benson.potionshop.service.MessageService;
import com.benson.potionshop.service.ShopService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.*;

import java.util.HashMap;
import java.util.Map;

/**
 * REST Controller for the Potion Shop API
 */
@Controller
public class ShopController {

    @Autowired
    private ShopService shopService;

    @Autowired
    private MessageService messageService;

    /**
     * Serve the main page
     */
    @GetMapping("/")
    public String index() {
        return "index";
    }

    /**
     * POST /api/buy - Purchase a potion
     * Increments stats and returns a random message
     */
    @PostMapping("/api/buy")
    @ResponseBody
    public Map<String, Object> buyPotion() {
        ShopStats stats = shopService.incrementSales(50);
        String message = messageService.getRandomMessage();

        Map<String, Object> response = new HashMap<>();
        response.put("success", true);
        response.put("message", message);
        response.put("stats", convertStatsToMap(stats));

        return response;
    }

    /**
     * GET /api/stats - Get shop statistics
     */
    @GetMapping("/api/stats")
    @ResponseBody
    public Map<String, Object> getStats() {
        ShopStats stats = shopService.getStats();

        Map<String, Object> response = new HashMap<>();
        response.put("potions_sold", stats.getPotionsSold());
        response.put("gold_earned", stats.getGoldEarned());
        response.put("gold_per_potion", 50);

        return response;
    }

    /**
     * POST /api/reset - Reset statistics
     */
    @PostMapping("/api/reset")
    @ResponseBody
    public Map<String, Object> resetStats() {
        ShopStats stats = shopService.reset();

        Map<String, Object> response = new HashMap<>();
        response.put("success", true);
        response.put("message", "Shop statistics have been reset");
        response.put("stats", convertStatsToMap(stats));

        return response;
    }

    /**
     * GET /api/health - Health check
     */
    @GetMapping("/api/health")
    @ResponseBody
    public Map<String, Object> healthCheck() {
        Map<String, Object> response = new HashMap<>();
        response.put("status", "healthy");
        response.put("shop", "open");
        response.put("potions_available", true);

        return response;
    }

    /**
     * Helper method to convert ShopStats to Map
     */
    private Map<String, Integer> convertStatsToMap(ShopStats stats) {
        Map<String, Integer> statsMap = new HashMap<>();
        statsMap.put("potions_sold", stats.getPotionsSold());
        statsMap.put("gold_earned", stats.getGoldEarned());
        return statsMap;
    }
}
