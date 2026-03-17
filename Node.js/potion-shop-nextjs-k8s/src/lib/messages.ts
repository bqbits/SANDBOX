/**
 * Fun messages for potion purchases
 */

export const PURCHASE_MESSAGES = [
  "You got a Red Potion! Your hearts are refilled!",
  "A fine purchase, brave hero! May it serve you well!",
  "One Red Potion coming right up! Stay safe out there!",
  "Excellent choice! This potion has saved many adventurers!",
  "A wise investment! You'll thank yourself later!",
  "Thank you for your patronage, noble warrior!",
  "It's dangerous to go alone! Take this potion!",
  "May this potion guide you to victory!",
];

export const getRandomMessage = (): string => {
  return PURCHASE_MESSAGES[Math.floor(Math.random() * PURCHASE_MESSAGES.length)];
};
