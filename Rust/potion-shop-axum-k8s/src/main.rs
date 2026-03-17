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
use tower_http::cors::CorsLayer;

async fn index() -> Html<String> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Benson's Potion Shop</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap" rel="stylesheet">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
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

        header img.banner {
            max-width: 400px;
            width: 90%;
            margin-bottom: 10px;
        }

        header p.tagline {
            font-size: 8px;
            color: #a0a060;
            margin-top: 8px;
            letter-spacing: 2px;
        }

        .stats-panel {
            background: rgba(0, 0, 0, 0.5);
            border: 3px solid #f0c040;
            border-radius: 8px;
            padding: 16px 24px;
            max-width: 500px;
            margin: 0 auto 30px;
            display: flex;
            justify-content: space-around;
            gap: 12px;
            flex-wrap: wrap;
        }

        .stat-item {
            text-align: center;
            animation: pulse-stats 3s infinite ease-in-out;
        }

        .stat-item .stat-label {
            font-size: 6px;
            color: #8b6914;
            display: block;
            margin-bottom: 8px;
            letter-spacing: 1px;
        }

        .stat-item .stat-value {
            font-size: 14px;
            color: #f0c040;
            display: block;
        }

        @keyframes pulse-stats {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.8; }
        }

        .shop-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
            gap: 24px;
            max-width: 800px;
            margin: 0 auto 30px;
        }

        .potion-card {
            background: rgba(0, 20, 0, 0.7);
            border: 3px solid #8b6914;
            border-radius: 10px;
            padding: 24px 16px;
            text-align: center;
            transition: border-color 0.3s, transform 0.2s;
            position: relative;
            overflow: hidden;
        }

        .potion-card:hover {
            border-color: #f0c040;
            transform: translateY(-4px);
        }

        .potion-card::before {
            content: '';
            position: absolute;
            top: -50%;
            left: -50%;
            width: 200%;
            height: 200%;
            background: radial-gradient(circle, rgba(240,192,64,0.05) 0%, transparent 60%);
            animation: sparkle 4s infinite linear;
        }

        @keyframes sparkle {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }

        .potion-img-wrap {
            display: inline-block;
            animation: float-potion 3s ease-in-out infinite;
            margin-bottom: 16px;
        }

        @keyframes float-potion {
            0%, 100% { transform: translateY(0px); }
            50% { transform: translateY(-10px); }
        }

        .potion-img-wrap img {
            width: 60px;
            height: 80px;
        }

        .potion-name {
            font-size: 10px;
            color: #f0c040;
            margin-bottom: 10px;
            display: block;
        }

        .potion-desc {
            font-size: 6px;
            color: #a0c060;
            margin-bottom: 16px;
            line-height: 1.8;
            display: block;
        }

        .potion-price {
            font-size: 9px;
            color: #f0c040;
            margin-bottom: 18px;
            display: block;
        }

        .potion-price::before {
            content: '💰 ';
        }

        .btn-buy {
            font-family: 'Press Start 2P', monospace;
            background: #8b6914;
            color: #f0c040;
            border: 2px solid #f0c040;
            padding: 10px 18px;
            font-size: 8px;
            cursor: pointer;
            border-radius: 4px;
            transition: background 0.2s, transform 0.1s;
            width: 100%;
        }

        .btn-buy:hover {
            background: #a07820;
            transform: scale(1.04);
        }

        .btn-buy:active {
            transform: scale(0.97);
        }

        .message-box {
            background: rgba(0, 40, 0, 0.9);
            border: 3px solid #f0c040;
            border-radius: 8px;
            padding: 16px 24px;
            max-width: 600px;
            margin: 0 auto 20px;
            text-align: center;
            font-size: 9px;
            line-height: 1.8;
            color: #c8e868;
            display: none;
            animation: fadeIn 0.4s ease-in;
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(-8px); }
            to   { opacity: 1; transform: translateY(0); }
        }

        .message-box.visible {
            display: block;
        }

        .actions {
            text-align: center;
            margin-bottom: 40px;
        }

        .btn-reset {
            font-family: 'Press Start 2P', monospace;
            background: transparent;
            color: #cc4444;
            border: 2px solid #cc4444;
            padding: 10px 20px;
            font-size: 8px;
            cursor: pointer;
            border-radius: 4px;
            transition: background 0.2s, color 0.2s;
        }

        .btn-reset:hover {
            background: #cc4444;
            color: #fff;
        }

        footer {
            text-align: center;
            font-size: 6px;
            color: #3a4a2a;
            padding-bottom: 20px;
            letter-spacing: 2px;
        }

        @media (max-width: 480px) {
            .shop-grid {
                grid-template-columns: 1fr;
            }
            .stats-panel {
                flex-direction: column;
                align-items: center;
            }
        }
    </style>
</head>
<body>

<header>
    <img class="banner" src="/static/banner.svg" alt="Benson's Potion Shop">
    <p class="tagline">Est. Year of the Hero &nbsp;|&nbsp; Hyrule Market District</p>
</header>

<div class="stats-panel" id="stats-panel">
    <div class="stat-item">
        <span class="stat-label">POTIONS SOLD</span>
        <span class="stat-value" id="stat-sold">0</span>
    </div>
    <div class="stat-item">
        <span class="stat-label">GOLD EARNED</span>
        <span class="stat-value" id="stat-gold">0</span>
    </div>
    <div class="stat-item">
        <span class="stat-label">PRICE / POTION</span>
        <span class="stat-value" id="stat-price">50</span>
    </div>
</div>

<div class="message-box" id="message-box"></div>

<div class="shop-grid">
    <div class="potion-card">
        <div class="potion-img-wrap">
            <img src="/static/red-potion.svg" alt="Red Potion">
        </div>
        <span class="potion-name">Red Potion</span>
        <span class="potion-desc">Restores all heart<br>containers instantly.</span>
        <span class="potion-price">50 Gold</span>
        <button class="btn-buy" onclick="buyPotion()">Buy Potion</button>
    </div>
    <div class="potion-card">
        <div class="potion-img-wrap">
            <img src="/static/red-potion.svg" alt="Red Potion">
        </div>
        <span class="potion-name">Red Potion</span>
        <span class="potion-desc">Restores all heart<br>containers instantly.</span>
        <span class="potion-price">50 Gold</span>
        <button class="btn-buy" onclick="buyPotion()">Buy Potion</button>
    </div>
    <div class="potion-card">
        <div class="potion-img-wrap">
            <img src="/static/red-potion.svg" alt="Red Potion">
        </div>
        <span class="potion-name">Red Potion</span>
        <span class="potion-desc">Restores all heart<br>containers instantly.</span>
        <span class="potion-price">50 Gold</span>
        <button class="btn-buy" onclick="buyPotion()">Buy Potion</button>
    </div>
</div>

<div class="actions">
    <button class="btn-reset" onclick="resetStats()">Reset Stats</button>
</div>

<footer>
    &copy; BENSON'S POTION SHOP &nbsp;|&nbsp; ALL RIGHTS RESERVED
</footer>

<script>
    let msgTimer = null;

    async function loadStats() {
        try {
            const res = await fetch('/api/stats');
            const data = await res.json();
            updateStatsDisplay(data);
        } catch (e) {
            console.error('Failed to load stats:', e);
        }
    }

    function updateStatsDisplay(stats) {
        document.getElementById('stat-sold').textContent = stats.potions_sold;
        document.getElementById('stat-gold').textContent = stats.gold_earned;
        document.getElementById('stat-price').textContent = stats.gold_per_potion;
    }

    function showMessage(msg) {
        const box = document.getElementById('message-box');
        box.textContent = msg;
        box.classList.add('visible');
        if (msgTimer) clearTimeout(msgTimer);
        msgTimer = setTimeout(() => {
            box.classList.remove('visible');
        }, 4000);
    }

    async function buyPotion() {
        try {
            const res = await fetch('/api/buy', { method: 'POST' });
            const data = await res.json();
            if (data.success) {
                updateStatsDisplay(data.stats);
                showMessage(data.message);
            }
        } catch (e) {
            showMessage('The shop keeper is busy... try again!');
        }
    }

    async function resetStats() {
        try {
            const res = await fetch('/api/reset', { method: 'POST' });
            const data = await res.json();
            if (data.success) {
                updateStatsDisplay(data.stats);
                showMessage(data.message);
            }
        } catch (e) {
            console.error('Failed to reset:', e);
        }
    }

    loadStats();
</script>
</body>
</html>"#;
    Html(html.to_string())
}

async fn buy_potion(State(store): State<SharedStore>) -> Json<Value> {
    let messages = [
        "You got a Red Potion! Your hearts are refilled!",
        "A fine purchase, brave hero! May it serve you well!",
        "One Red Potion coming right up! Stay safe out there!",
        "Excellent choice! This potion has saved many adventurers!",
        "A wise investment! You'll thank yourself later!",
        "Thank you for your patronage, noble warrior!",
        "It's dangerous to go alone! Take this potion!",
        "May this potion guide you to victory!",
    ];

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_millis() as usize;
    let idx = millis % 8;
    let message = messages[idx];

    let stats = {
        let mut s = store.lock().unwrap();
        s.potions_sold += 1;
        s.gold_earned += s.gold_per_potion;
        s.clone()
    };

    Json(json!({
        "success": true,
        "message": message,
        "stats": stats
    }))
}

async fn get_stats(State(store): State<SharedStore>) -> Json<Value> {
    let stats = store.lock().unwrap().clone();
    Json(json!(stats))
}

async fn reset_stats(State(store): State<SharedStore>) -> Json<Value> {
    let stats = {
        let mut s = store.lock().unwrap();
        s.potions_sold = 0;
        s.gold_earned = 0;
        s.clone()
    };

    Json(json!({
        "success": true,
        "message": "Stats reset!",
        "stats": stats
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "shop": "open",
        "potions_available": true
    }))
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let store = new_store();

    let app = Router::new()
        .route("/", get(index))
        .route("/api/buy", post(buy_potion))
        .route("/api/stats", get(get_stats))
        .route("/api/reset", post(reset_stats))
        .route("/api/health", get(health_check))
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(store);

    let addr = format!("0.0.0.0:{port}");
    println!("Potion shop is open at http://0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
