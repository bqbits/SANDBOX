'use client';

import { useState, useEffect } from 'react';
import Image from 'next/image';

interface ShopStats {
  potions_sold: number;
  gold_earned: number;
}

export default function Shop() {
  const [stats, setStats] = useState<ShopStats>({ potions_sold: 0, gold_earned: 0 });
  const [message, setMessage] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  // Fetch initial stats
  useEffect(() => {
    fetchStats();
  }, []);

  const fetchStats = async () => {
    try {
      const response = await fetch('/api/stats');
      const data = await response.json();
      setStats({
        potions_sold: data.potions_sold,
        gold_earned: data.gold_earned,
      });
    } catch (error) {
      console.error('Failed to fetch stats:', error);
    }
  };

  const buyPotion = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/buy', {
        method: 'POST',
      });
      const data = await response.json();

      if (data.success) {
        setMessage(data.message);
        setStats(data.stats);

        // Clear message after 4 seconds
        setTimeout(() => {
          setMessage(null);
        }, 4000);
      }
    } catch (error) {
      console.error('Failed to buy potion:', error);
    } finally {
      setLoading(false);
    }
  };

  const resetStats = async () => {
    try {
      const response = await fetch('/api/reset', {
        method: 'POST',
      });
      const data = await response.json();

      if (data.success) {
        setStats(data.stats);
        setMessage(null);
      }
    } catch (error) {
      console.error('Failed to reset stats:', error);
    }
  };

  return (
    <div className="container">
      <div className="banner">
        <Image
          src="/images/banner.svg"
          alt="Benson's Potion Store"
          width={800}
          height={200}
          priority
        />
      </div>

      <div className="welcome-message">
        <p>⚔️ Welcome, brave adventurer! ⚔️</p>
        <p>You have stumbled upon the finest potion shop in all of Hyrule!</p>
        <p>Our potions are brewed with care and tested by heroes!</p>
      </div>

      {message && <div className="message">{message}</div>}

      <div className="shop-grid">
        <div className="potion-card">
          <Image
            src="/images/red-potion.svg"
            alt="Red Potion"
            width={120}
            height={120}
            className="potion-image"
          />
          <h2 className="potion-name">RED POTION</h2>
          <p className="potion-description">
            Restores all your <span className="heart">❤️</span> hearts!
            <br />
            <br />
            A legendary elixir brewed by the finest alchemists. Perfect for
            dungeon crawling and boss battles!
          </p>
          <div className="price">
            <span className="gold-coin">⭐</span> 50 GOLD{' '}
            <span className="gold-coin">⭐</span>
          </div>
          <button
            onClick={buyPotion}
            disabled={loading}
            className="buy-button"
          >
            {loading ? 'BUYING...' : 'BUY NOW'}
          </button>
        </div>

        <div className="potion-card">
          <Image
            src="/images/red-potion.svg"
            alt="Red Potion"
            width={120}
            height={120}
            className="potion-image"
          />
          <h2 className="potion-name">RED POTION</h2>
          <p className="potion-description">
            Restores all your <span className="heart">❤️</span> hearts!
            <br />
            <br />
            Stock up for your adventure! Every hero needs backup potions.
          </p>
          <div className="price">
            <span className="gold-coin">⭐</span> 50 GOLD{' '}
            <span className="gold-coin">⭐</span>
          </div>
          <button
            onClick={buyPotion}
            disabled={loading}
            className="buy-button"
          >
            {loading ? 'BUYING...' : 'BUY NOW'}
          </button>
        </div>

        <div className="potion-card">
          <Image
            src="/images/red-potion.svg"
            alt="Red Potion"
            width={120}
            height={120}
            className="potion-image"
          />
          <h2 className="potion-name">RED POTION</h2>
          <p className="potion-description">
            Restores all your <span className="heart">❤️</span> hearts!
            <br />
            <br />
            The third potion is always the lucky charm! Don&apos;t miss out!
          </p>
          <div className="price">
            <span className="gold-coin">⭐</span> 50 GOLD{' '}
            <span className="gold-coin">⭐</span>
          </div>
          <button
            onClick={buyPotion}
            disabled={loading}
            className="buy-button"
          >
            {loading ? 'BUYING...' : 'BUY NOW'}
          </button>
        </div>
      </div>

      <div className="stats-footer">
        <p>🗡️ Total Potions Sold: {stats.potions_sold} 🗡️</p>
        <p>💰 Total Gold Earned: {stats.gold_earned} 💰</p>
        <p className="quote">
          &quot;It&apos;s dangerous to go alone! Take a potion!&quot; - Old Man,
          probably
        </p>
        <button onClick={resetStats} className="reset-button">
          RESET STATS
        </button>
      </div>
    </div>
  );
}
