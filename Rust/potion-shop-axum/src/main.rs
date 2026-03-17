mod store;

use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use store::{new_store, SharedStore};
use tower_http::cors::{Any, CorsLayer};

const PURCHASE_MESSAGES: [&str; 8] = [
    "You got a Red Potion! Your hearts are refilled!",
    "A fine purchase, brave hero! May it serve you well!",
    "One Red Potion coming right up! Stay safe out there!",
    "Excellent choice! This potion has saved many adventurers!",
    "A wise investment! You'll thank yourself later!",
    "Thank you for your patronage, noble warrior!",
    "It's dangerous to go alone! Take this potion!",
    "May this potion guide you to victory!",
];

fn pick_message() -> &'static str {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_millis() as u64;
    let idx = (millis % 8) as usize;
    PURCHASE_MESSAGES[idx]
}

async fn root_handler() -> Html<&'static str> {
    Html(HTML_PAGE)
}

async fn buy_handler(State(store): State<SharedStore>) -> Json<Value> {
    let stats = {
        let mut s = store.lock().unwrap();
        s.potions_sold += 1;
        s.gold_earned += s.gold_per_potion;
        s.clone()
    };
    let message = pick_message();
    Json(json!({
        "success": true,
        "message": message,
        "stats": {
            "potions_sold": stats.potions_sold,
            "gold_earned": stats.gold_earned,
            "gold_per_potion": stats.gold_per_potion,
        }
    }))
}

async fn stats_handler(State(store): State<SharedStore>) -> Json<Value> {
    let s = store.lock().unwrap();
    Json(json!({
        "potions_sold": s.potions_sold,
        "gold_earned": s.gold_earned,
        "gold_per_potion": s.gold_per_potion,
    }))
}

async fn reset_handler(State(store): State<SharedStore>) -> Json<Value> {
    let stats = {
        let mut s = store.lock().unwrap();
        s.potions_sold = 0;
        s.gold_earned = 0;
        s.clone()
    };
    Json(json!({
        "success": true,
        "message": "Stats reset!",
        "stats": {
            "potions_sold": stats.potions_sold,
            "gold_earned": stats.gold_earned,
            "gold_per_potion": stats.gold_per_potion,
        }
    }))
}

async fn health_handler() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "shop": "open",
        "potions_available": true,
    }))
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let store = new_store();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/buy", post(buy_handler))
        .route("/api/stats", get(stats_handler))
        .route("/api/reset", post(reset_handler))
        .route("/api/health", get(health_handler))
        .with_state(store)
        .layer(cors);

    let addr = format!("0.0.0.0:{port}");
    println!("Benson's Potion Shop is open at http://localhost:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

static HTML_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Benson's Potion Shop</title>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap" rel="stylesheet" />
  <style>
    *, *::before, *::after {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }

    body {
      font-family: 'Press Start 2P', monospace;
      background: linear-gradient(135deg, #0a1a0a 0%, #1a2e1a 100%);
      min-height: 100vh;
      color: #f0c040;
      padding: 20px;
    }

    header {
      text-align: center;
      padding: 30px 20px 20px;
    }

    header h1 {
      font-size: clamp(14px, 3vw, 22px);
      color: #f0c040;
      text-shadow: 2px 2px 0 #8b6914, 4px 4px 0 #5a3e00;
      letter-spacing: 2px;
      margin-bottom: 8px;
    }

    header p.subtitle {
      font-size: 9px;
      color: #a0d0a0;
      letter-spacing: 1px;
    }

    /* Stats panel */
    .stats-panel {
      display: flex;
      justify-content: center;
      gap: 20px;
      flex-wrap: wrap;
      margin: 24px auto;
      max-width: 700px;
    }

    .stat-card {
      background: #0d1f0d;
      border: 3px solid #f0c040;
      border-radius: 6px;
      padding: 14px 22px;
      text-align: center;
      animation: pulse-stat 3s ease-in-out infinite;
      min-width: 160px;
    }

    .stat-card:nth-child(2) { animation-delay: 1.5s; }

    @keyframes pulse-stat {
      0%, 100% { box-shadow: 0 0 6px #f0c04066; }
      50%       { box-shadow: 0 0 18px #f0c040cc; }
    }

    .stat-label {
      font-size: 7px;
      color: #a0d0a0;
      margin-bottom: 8px;
      letter-spacing: 1px;
    }

    .stat-value {
      font-size: 18px;
      color: #f0c040;
      text-shadow: 1px 1px 0 #8b6914;
    }

    /* Message box */
    .message-box {
      display: none;
      max-width: 640px;
      margin: 0 auto 24px;
      background: #1a2e1a;
      border: 3px solid #f0c040;
      border-radius: 6px;
      padding: 14px 20px;
      text-align: center;
      font-size: 9px;
      color: #f0f0c0;
      line-height: 1.8;
      animation: sparkle 0.4s ease-out;
    }

    .message-box.visible { display: block; }

    @keyframes sparkle {
      0%   { transform: scale(0.92); opacity: 0; }
      60%  { transform: scale(1.03); opacity: 1; }
      100% { transform: scale(1);    opacity: 1; }
    }

    /* Potion grid */
    .potion-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
      gap: 24px;
      max-width: 780px;
      margin: 0 auto 32px;
    }

    .potion-card {
      background: #0d1f0d;
      border: 3px solid #8b6914;
      border-radius: 8px;
      padding: 24px 20px;
      text-align: center;
      transition: border-color 0.2s, transform 0.2s;
    }

    .potion-card:hover {
      border-color: #f0c040;
      transform: translateY(-4px);
    }

    .potion-icon {
      font-size: 52px;
      display: block;
      margin-bottom: 12px;
      animation: float 3s ease-in-out infinite;
    }

    .potion-card:nth-child(2) .potion-icon { animation-delay: 1s; }
    .potion-card:nth-child(3) .potion-icon { animation-delay: 2s; }

    @keyframes float {
      0%, 100% { transform: translateY(0); }
      50%       { transform: translateY(-10px); }
    }

    .potion-name {
      font-size: 10px;
      color: #f0c040;
      margin-bottom: 8px;
    }

    .potion-price {
      font-size: 9px;
      color: #a0d0a0;
      margin-bottom: 18px;
    }

    .btn-buy {
      font-family: 'Press Start 2P', monospace;
      font-size: 8px;
      background: #8b6914;
      color: #f0c040;
      border: 3px solid #f0c040;
      border-radius: 4px;
      padding: 10px 16px;
      cursor: pointer;
      transition: background 0.15s, transform 0.1s;
      width: 100%;
      letter-spacing: 1px;
    }

    .btn-buy:hover  { background: #a07820; }
    .btn-buy:active { transform: scale(0.96); }

    /* Footer / reset */
    .footer-controls {
      text-align: center;
      margin-bottom: 40px;
    }

    .btn-reset {
      font-family: 'Press Start 2P', monospace;
      font-size: 8px;
      background: transparent;
      color: #cc4444;
      border: 2px solid #cc4444;
      border-radius: 4px;
      padding: 10px 20px;
      cursor: pointer;
      letter-spacing: 1px;
      transition: background 0.15s, color 0.15s;
    }

    .btn-reset:hover {
      background: #cc4444;
      color: #fff;
    }

    footer {
      text-align: center;
      font-size: 7px;
      color: #4a6a4a;
      padding-bottom: 20px;
      letter-spacing: 1px;
    }
  </style>
</head>
<body>

  <header>
    <h1>Benson's Potion Shop</h1>
    <p class="subtitle">* Welcome, brave adventurer! *</p>
  </header>

  <!-- Stats -->
  <div class="stats-panel">
    <div class="stat-card">
      <div class="stat-label">Potions Sold</div>
      <div class="stat-value" id="stat-sold">0</div>
    </div>
    <div class="stat-card">
      <div class="stat-label">Gold Earned</div>
      <div class="stat-value" id="stat-gold">0</div>
    </div>
  </div>

  <!-- Message -->
  <div class="message-box" id="message-box"></div>

  <!-- Potion cards -->
  <div class="potion-grid">
    <div class="potion-card">
      <span class="potion-icon">&#x1F9EA;</span>
      <div class="potion-name">Red Potion</div>
      <div class="potion-price">50 Gold</div>
      <button class="btn-buy" onclick="buyPotion()">Buy Potion</button>
    </div>
    <div class="potion-card">
      <span class="potion-icon">&#x1F9EA;</span>
      <div class="potion-name">Red Potion</div>
      <div class="potion-price">50 Gold</div>
      <button class="btn-buy" onclick="buyPotion()">Buy Potion</button>
    </div>
    <div class="potion-card">
      <span class="potion-icon">&#x1F9EA;</span>
      <div class="potion-name">Red Potion</div>
      <div class="potion-price">50 Gold</div>
      <button class="btn-buy" onclick="buyPotion()">Buy Potion</button>
    </div>
  </div>

  <!-- Reset -->
  <div class="footer-controls">
    <button class="btn-reset" onclick="resetStats()">Reset Stats</button>
  </div>

  <footer>
    &copy; Benson's Potion Shop &mdash; Built with Rust &amp; Axum
  </footer>

  <script>
    let hideTimer = null;

    function updateStats(stats) {
      document.getElementById('stat-sold').textContent = stats.potions_sold;
      document.getElementById('stat-gold').textContent = stats.gold_earned;
    }

    function showMessage(text) {
      const box = document.getElementById('message-box');
      box.textContent = text;
      box.classList.add('visible');
      if (hideTimer) clearTimeout(hideTimer);
      hideTimer = setTimeout(() => box.classList.remove('visible'), 4000);
    }

    async function loadStats() {
      try {
        const res = await fetch('/api/stats');
        const data = await res.json();
        updateStats(data);
      } catch (e) {
        console.error('Failed to load stats:', e);
      }
    }

    async function buyPotion() {
      try {
        const res = await fetch('/api/buy', { method: 'POST' });
        const data = await res.json();
        if (data.success) {
          updateStats(data.stats);
          showMessage(data.message);
        }
      } catch (e) {
        console.error('Failed to buy potion:', e);
      }
    }

    async function resetStats() {
      try {
        const res = await fetch('/api/reset', { method: 'POST' });
        const data = await res.json();
        if (data.success) {
          updateStats(data.stats);
          showMessage(data.message);
        }
      } catch (e) {
        console.error('Failed to reset stats:', e);
      }
    }

    // Load stats on page load
    loadStats();
  </script>
</body>
</html>
"#;
