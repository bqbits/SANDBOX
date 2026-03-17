class CreateShopStats < ActiveRecord::Migration[8.1]
  def change
    create_table :shop_stats do |t|
      t.integer :potions_sold, default: 0, null: false
      t.integer :gold_earned, default: 0, null: false

      t.timestamps
    end
  end
end
