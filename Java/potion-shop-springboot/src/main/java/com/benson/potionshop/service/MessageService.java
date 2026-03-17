package com.benson.potionshop.service;

import org.springframework.stereotype.Service;
import java.util.Random;

/**
 * Service for generating random purchase messages
 */
@Service
public class MessageService {

    private static final String[] PURCHASE_MESSAGES = {
        "You got a Red Potion! Your hearts are refilled!",
        "A fine purchase, brave hero! May it serve you well!",
        "One Red Potion coming right up! Stay safe out there!",
        "Excellent choice! This potion has saved many adventurers!",
        "A wise investment! You'll thank yourself later!",
        "Thank you for your patronage, noble warrior!",
        "It's dangerous to go alone! Take this potion!",
        "May this potion guide you to victory!"
    };

    private final Random random = new Random();

    public String getRandomMessage() {
        return PURCHASE_MESSAGES[random.nextInt(PURCHASE_MESSAGES.length)];
    }
}
