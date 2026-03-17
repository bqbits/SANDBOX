#!/usr/bin/env python3
"""
Benson's Potion Store - A Zelda-inspired Flask Shop
A lightweight storefront for testing Datadog APM Python tracer
"""

from flask import Flask, render_template, request, redirect, url_for
import random

app = Flask(__name__)

# Simple in-memory storage for shop statistics
shop_stats = {
    'potions_sold': 0,
    'gold_earned': 0
}

# Fun messages for purchases
PURCHASE_MESSAGES = [
    "🎉 You got a Red Potion! Your hearts are refilled! 🎉",
    "✨ A fine purchase, brave hero! May it serve you well! ✨",
    "🗡️ One Red Potion coming right up! Stay safe out there! 🗡️",
    "⚔️ Excellent choice! This potion has saved many adventurers! ⚔️",
    "🏆 A wise investment! You'll thank yourself later! 🏆",
    "🎪 Thank you for your patronage, noble warrior! 🎪",
    "💫 It's dangerous to go alone! Take this potion! 💫",
    "🌟 May this potion guide you to victory! 🌟",
]


@app.route('/')
def shop():
    """Display the main potion shop"""
    return render_template(
        'shop.html',
        potions_sold=shop_stats['potions_sold'],
        gold_earned=shop_stats['gold_earned'],
        message=None
    )


@app.route('/buy', methods=['POST'])
def buy_potion():
    """Handle potion purchase"""
    potion_type = request.form.get('potion', 'red')

    # Update shop statistics
    shop_stats['potions_sold'] += 1
    shop_stats['gold_earned'] += 50  # Red potion costs 50 gold

    # Select a random fun message
    message = random.choice(PURCHASE_MESSAGES)

    return render_template(
        'shop.html',
        potions_sold=shop_stats['potions_sold'],
        gold_earned=shop_stats['gold_earned'],
        message=message
    )


@app.route('/stats')
def stats():
    """API endpoint for shop statistics"""
    return {
        'potions_sold': shop_stats['potions_sold'],
        'gold_earned': shop_stats['gold_earned'],
        'gold_per_potion': 50
    }


@app.route('/reset', methods=['POST'])
def reset_stats():
    """Reset shop statistics (for testing)"""
    shop_stats['potions_sold'] = 0
    shop_stats['gold_earned'] = 0
    return redirect(url_for('shop'))


@app.route('/health')
def health():
    """Health check endpoint for monitoring"""
    return {'status': 'healthy', 'shop': 'open', 'potions_available': True}


if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
