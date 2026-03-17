class Api::ShopController < ApplicationController
  skip_before_action :verify_authenticity_token

  POTION_COST = 50
  PURCHASE_MESSAGES = [
    "It's dangerous to go alone! Take this.",
    "You got a Red Potion! Your health is restored!",
    "This will help you on your quest, traveler.",
    "A fine choice! That'll be 50 gold.",
    "Thank you for your business, hero!",
    "May this potion serve you well in battle!",
    "One Red Potion, coming right up!",
    "You look like you could use this, adventurer."
  ]

  def buy
    stat = ShopStat.current_stats
    stat.add_purchase(POTION_COST)

    message = PURCHASE_MESSAGES.sample

    render json: {
      message: message,
      potions_sold: stat.potions_sold,
      gold_earned: stat.gold_earned
    }
  end

  def stats
    stat = ShopStat.current_stats

    render json: {
      potions_sold: stat.potions_sold,
      gold_earned: stat.gold_earned
    }
  end

  def reset
    stat = ShopStat.current_stats
    stat.reset_stats

    render json: {
      message: "Shop statistics have been reset!",
      potions_sold: stat.potions_sold,
      gold_earned: stat.gold_earned
    }
  end

  def health
    render json: {
      status: "ok",
      timestamp: Time.current.iso8601,
      service: "potion-shop-rails-k8s"
    }
  end
end
