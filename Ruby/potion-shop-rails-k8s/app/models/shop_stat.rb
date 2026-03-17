class ShopStat < ApplicationRecord
  validates :potions_sold, :gold_earned, numericality: { greater_than_or_equal_to: 0 }

  def self.current_stats
    first_or_create(potions_sold: 0, gold_earned: 0)
  end

  def add_purchase(potion_cost)
    self.potions_sold += 1
    self.gold_earned += potion_cost
    save
  end

  def reset_stats
    self.potions_sold = 0
    self.gold_earned = 0
    save
  end
end
