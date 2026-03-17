package com.benson.potionshop.controller;

import com.benson.potionshop.model.ShopStats;
import com.benson.potionshop.service.MessageService;
import com.benson.potionshop.service.ShopService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/api")
public class ShopController {

    @Autowired
    private ShopService shopService;

    @Autowired
    private MessageService messageService;

    @PostMapping("/buy")
    public Map<String, Object> buyPotion() {
        ShopStats stats = shopService.purchasePotion();
        String message = messageService.getRandomPurchaseMessage();

        Map<String, Object> response = new HashMap<>();
        response.put("success", true);
        response.put("message", message);

        Map<String, Integer> statsMap = new HashMap<>();
        statsMap.put("potions_sold", stats.getPotionsSold());
        statsMap.put("gold_earned", stats.getGoldEarned());
        response.put("stats", statsMap);

        return response;
    }

    @GetMapping("/stats")
    public Map<String, Integer> getStats() {
        ShopStats stats = shopService.getStats();

        Map<String, Integer> response = new HashMap<>();
        response.put("potions_sold", stats.getPotionsSold());
        response.put("gold_earned", stats.getGoldEarned());
        response.put("gold_per_potion", shopService.getPotionPrice());

        return response;
    }

    @PostMapping("/reset")
    public Map<String, Object> resetStats() {
        ShopStats stats = shopService.resetStats();

        Map<String, Object> response = new HashMap<>();
        response.put("success", true);
        response.put("message", "Shop statistics have been reset");

        Map<String, Integer> statsMap = new HashMap<>();
        statsMap.put("potions_sold", stats.getPotionsSold());
        statsMap.put("gold_earned", stats.getGoldEarned());
        response.put("stats", statsMap);

        return response;
    }

    @GetMapping("/health")
    public Map<String, Object> health() {
        Map<String, Object> response = new HashMap<>();
        response.put("status", "healthy");
        response.put("shop", "open");
        response.put("potions_available", true);

        return response;
    }
}
