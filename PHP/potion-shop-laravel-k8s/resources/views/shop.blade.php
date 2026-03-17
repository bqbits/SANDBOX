<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="csrf-token" content="{{ csrf_token() }}">
    <title>Benson's Potion Store</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            background: linear-gradient(180deg, #1a472a 0%, #0d2818 100%);
            color: #f0e68c;
            min-height: 100vh;
            padding: 20px;
            font-family: 'Courier New', monospace;
        }

        .container {
            max-width: 900px;
            margin: 0 auto;
        }

        .banner {
            text-align: center;
            margin-bottom: 40px;
            animation: float 3s ease-in-out infinite;
        }

        .banner h1 {
            font-size: 48px;
            color: #ffd700;
            text-shadow: 4px 4px 8px rgba(0, 0, 0, 0.8);
            letter-spacing: 4px;
        }

        @keyframes float {
            0%, 100% {
                transform: translateY(0px);
            }
            50% {
                transform: translateY(-10px);
            }
        }

        .welcome-message {
            background: rgba(139, 69, 19, 0.8);
            border: 4px solid #654321;
            padding: 20px;
            margin-bottom: 30px;
            text-align: center;
            font-size: 14px;
            line-height: 1.8;
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
        }

        .welcome-message p {
            margin-top: 15px;
        }

        .welcome-message p:first-child {
            margin-top: 0;
        }

        .shop-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 30px;
            margin: 40px 0;
        }

        .potion-card {
            background: rgba(101, 67, 33, 0.9);
            border: 4px solid #ffd700;
            padding: 20px;
            text-align: center;
            transition: transform 0.3s, box-shadow 0.3s;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
            position: relative;
        }

        .potion-card:hover {
            transform: translateY(-10px) scale(1.05);
            box-shadow: 0 12px 24px rgba(255, 215, 0, 0.4);
            border-color: #ffa500;
        }

        .potion-card::before {
            content: "✨";
            position: absolute;
            top: 10px;
            left: 10px;
            font-size: 20px;
            animation: sparkle 2s infinite;
        }

        .potion-card::after {
            content: "✨";
            position: absolute;
            top: 10px;
            right: 10px;
            font-size: 20px;
            animation: sparkle 2s infinite 1s;
        }

        @keyframes sparkle {
            0%, 100% {
                opacity: 0.3;
                transform: scale(0.8);
            }
            50% {
                opacity: 1;
                transform: scale(1.2);
            }
        }

        .potion-image {
            width: 120px;
            height: 120px;
            margin: 20px auto;
            font-size: 80px;
        }

        .potion-name {
            font-size: 18px;
            margin: 15px 0;
            color: #ffd700;
            letter-spacing: 2px;
        }

        .potion-description {
            font-size: 12px;
            line-height: 1.6;
            margin: 15px 0;
            color: #f0e68c;
        }

        .price {
            font-size: 18px;
            color: #ffd700;
            margin: 20px 0;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
        }

        .gold-coin {
            display: inline-block;
            animation: rotate 3s linear infinite;
        }

        @keyframes rotate {
            0% {
                transform: rotateY(0deg);
            }
            100% {
                transform: rotateY(360deg);
            }
        }

        .buy-button {
            background: linear-gradient(180deg, #ffd700 0%, #ffa500 100%);
            border: 3px solid #ff8c00;
            padding: 12px 24px;
            font-family: inherit;
            font-size: 14px;
            font-weight: bold;
            color: #654321;
            cursor: pointer;
            transition: all 0.3s;
            box-shadow: 0 4px 0 #b8860b;
            letter-spacing: 1px;
        }

        .buy-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 0 #b8860b;
            background: linear-gradient(180deg, #ffa500 0%, #ffd700 100%);
        }

        .buy-button:active {
            transform: translateY(2px);
            box-shadow: 0 2px 0 #b8860b;
        }

        .buy-button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }

        .stats-footer {
            background: rgba(139, 69, 19, 0.8);
            border: 4px solid #654321;
            padding: 20px;
            margin-top: 40px;
            text-align: center;
            font-size: 14px;
            line-height: 1.8;
        }

        .stats-footer p {
            margin-top: 15px;
            font-size: 16px;
        }

        .stats-footer p:first-child {
            margin-top: 0;
        }

        .stats-footer .quote {
            margin-top: 20px;
            font-size: 12px;
            font-style: italic;
        }

        .message {
            background: rgba(255, 215, 0, 0.2);
            border: 3px solid #ffd700;
            padding: 15px;
            margin: 20px 0;
            font-size: 14px;
            text-align: center;
            animation: pulse 2s infinite;
        }

        @keyframes pulse {
            0%, 100% {
                opacity: 0.8;
            }
            50% {
                opacity: 1;
            }
        }

        .heart {
            color: #ff0000;
            display: inline-block;
            animation: heartbeat 1.5s infinite;
        }

        @keyframes heartbeat {
            0%, 100% {
                transform: scale(1);
            }
            25% {
                transform: scale(1.1);
            }
            50% {
                transform: scale(1);
            }
        }

        .reset-button {
            background: rgba(139, 69, 19, 0.6);
            border: 2px solid #654321;
            padding: 10px 20px;
            font-family: inherit;
            font-size: 12px;
            color: #f0e68c;
            cursor: pointer;
            transition: all 0.3s;
            margin-top: 15px;
        }

        .reset-button:hover {
            background: rgba(139, 69, 19, 0.9);
            border-color: #8b4513;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="banner">
            <h1>Benson's Potion Store</h1>
        </div>

        <div class="welcome-message">
            <p>⚔️ Welcome, brave adventurer! ⚔️</p>
            <p>You have stumbled upon the finest potion shop in all of Hyrule!</p>
            <p>Our potions are brewed with care and tested by heroes!</p>
        </div>

        <div id="message" class="message" style="display: none;"></div>

        <div class="shop-grid">
            <div class="potion-card">
                <div class="potion-image">🧪</div>
                <h2 class="potion-name">RED POTION</h2>
                <p class="potion-description">
                    Restores all your <span class="heart">❤️</span> hearts!
                    <br><br>
                    A legendary elixir brewed by the finest alchemists. Perfect for dungeon crawling and boss battles!
                </p>
                <div class="price">
                    <span class="gold-coin">⭐</span> 50 GOLD <span class="gold-coin">⭐</span>
                </div>
                <button onclick="buyPotion()" class="buy-button" id="buyButton1">
                    BUY NOW
                </button>
            </div>

            <div class="potion-card">
                <div class="potion-image">🧪</div>
                <h2 class="potion-name">RED POTION</h2>
                <p class="potion-description">
                    Restores all your <span class="heart">❤️</span> hearts!
                    <br><br>
                    Stock up for your adventure! Every hero needs backup potions.
                </p>
                <div class="price">
                    <span class="gold-coin">⭐</span> 50 GOLD <span class="gold-coin">⭐</span>
                </div>
                <button onclick="buyPotion()" class="buy-button" id="buyButton2">
                    BUY NOW
                </button>
            </div>

            <div class="potion-card">
                <div class="potion-image">🧪</div>
                <h2 class="potion-name">RED POTION</h2>
                <p class="potion-description">
                    Restores all your <span class="heart">❤️</span> hearts!
                    <br><br>
                    The third potion is always the lucky charm! Don't miss out!
                </p>
                <div class="price">
                    <span class="gold-coin">⭐</span> 50 GOLD <span class="gold-coin">⭐</span>
                </div>
                <button onclick="buyPotion()" class="buy-button" id="buyButton3">
                    BUY NOW
                </button>
            </div>
        </div>

        <div class="stats-footer">
            <p>🗡️ Total Potions Sold: <span id="potionsSold">0</span> 🗡️</p>
            <p>💰 Total Gold Earned: <span id="goldEarned">0</span> 💰</p>
            <p class="quote">
                "It's dangerous to go alone! Take a potion!" - Old Man, probably
            </p>
            <button onclick="resetStats()" class="reset-button">
                RESET STATS
            </button>
        </div>
    </div>

    <script>
        // Get CSRF token
        const csrfToken = document.querySelector('meta[name="csrf-token"]').getAttribute('content');

        // Fetch initial stats when page loads
        document.addEventListener('DOMContentLoaded', function() {
            fetchStats();
        });

        // Fetch stats from API
        async function fetchStats() {
            try {
                const response = await fetch('/api/stats');
                const data = await response.json();
                updateStats(data);
            } catch (error) {
                console.error('Failed to fetch stats:', error);
            }
        }

        // Buy potion
        async function buyPotion() {
            // Disable all buy buttons
            const buttons = document.querySelectorAll('.buy-button');
            buttons.forEach(btn => {
                btn.disabled = true;
                btn.textContent = 'BUYING...';
            });

            try {
                const response = await fetch('/api/buy', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'X-CSRF-TOKEN': csrfToken
                    }
                });
                const data = await response.json();

                if (data.success) {
                    showMessage(data.message);
                    updateStats(data.stats);
                }
            } catch (error) {
                console.error('Failed to buy potion:', error);
            } finally {
                // Re-enable all buy buttons
                buttons.forEach(btn => {
                    btn.disabled = false;
                    btn.textContent = 'BUY NOW';
                });
            }
        }

        // Reset stats
        async function resetStats() {
            try {
                const response = await fetch('/api/reset', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'X-CSRF-TOKEN': csrfToken
                    }
                });
                const data = await response.json();

                if (data.success) {
                    updateStats(data.stats);
                    hideMessage();
                }
            } catch (error) {
                console.error('Failed to reset stats:', error);
            }
        }

        // Update stats display
        function updateStats(stats) {
            document.getElementById('potionsSold').textContent = stats.potions_sold;
            document.getElementById('goldEarned').textContent = stats.gold_earned;
        }

        // Show message
        function showMessage(text) {
            const messageDiv = document.getElementById('message');
            messageDiv.textContent = text;
            messageDiv.style.display = 'block';

            // Hide message after 4 seconds
            setTimeout(() => {
                hideMessage();
            }, 4000);
        }

        // Hide message
        function hideMessage() {
            const messageDiv = document.getElementById('message');
            messageDiv.style.display = 'none';
        }
    </script>
</body>
</html>
