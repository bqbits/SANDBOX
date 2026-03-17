class ShopStat < ApplicationRecord
  validates :potions_sold, :gold_earned, numericality: { greater_than_or_equal_to: 0 }

  POTION_PRICE = 50

  PURCHASE_MESSAGES = [
    "It's dangerous to go alone! Take this.",
    "You got a Red Potion! Your health is restored!",
    "This will help you on your quest, traveler.",
    "A fine choice! That'll be 50 gold.",
    "Thank you for your business, hero!",
    "May this potion serve you well in battle!",
    "One Red Potion, coming right up!",
    "You look like you could use this, adventurer."
  ].freeze

  class << self
    def current_stats
      stat = first_or_create(potions_sold: 0, gold_earned: 0)
      {
        potions_sold: stat.potions_sold,
        gold_earned: stat.gold_earned
      }
    end

    def add_purchase
      stat = first_or_create(potions_sold: 0, gold_earned: 0)
      stat.increment!(:potions_sold)
      stat.increment!(:gold_earned, POTION_PRICE)
      {
        message: PURCHASE_MESSAGES.sample,
        potions_sold: stat.potions_sold,
        gold_earned: stat.gold_earned
      }
    end

    def reset_stats
      stat = first_or_create(potions_sold: 0, gold_earned: 0)
      stat.update(potions_sold: 0, gold_earned: 0)
      {
        potions_sold: 0,
        gold_earned: 0
      }
    end
  end
end
