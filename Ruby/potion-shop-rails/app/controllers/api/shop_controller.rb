class Api::ShopController < ApplicationController
  skip_before_action :verify_authenticity_token

  def buy
    result = ShopStat.add_purchase
    render json: result
  end

  def stats
    stats = ShopStat.current_stats
    render json: stats
  end

  def reset
    stats = ShopStat.reset_stats
    render json: {
      message: "Statistics have been reset!",
      stats: stats
    }
  end

  def health
    render json: {
      status: "ok",
      timestamp: Time.current.iso8601,
      service: "potion-shop-rails"
    }
  end
end
