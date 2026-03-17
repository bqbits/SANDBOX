// State management
let isLoading = false;

// Fetch and update stats on page load
document.addEventListener('DOMContentLoaded', () => {
    fetchStats();
});

// Fetch current shop statistics
async function fetchStats() {
    try {
        const response = await fetch('/api/stats');
        const data = await response.json();
        updateStatsDisplay(data);
    } catch (error) {
        console.error('Failed to fetch stats:', error);
    }
}

// Buy a potion
async function buyPotion() {
    if (isLoading) return;

    isLoading = true;
    disableButtons();

    try {
        const response = await fetch('/api/buy', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        const data = await response.json();

        if (data.success) {
            showMessage(data.message);
            updateStatsDisplay(data.stats);

            // Clear message after 4 seconds
            setTimeout(() => {
                hideMessage();
            }, 4000);
        }
    } catch (error) {
        console.error('Failed to buy potion:', error);
    } finally {
        isLoading = false;
        enableButtons();
    }
}

// Reset shop statistics
async function resetStats() {
    try {
        const response = await fetch('/api/reset', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        const data = await response.json();

        if (data.success) {
            updateStatsDisplay(data.stats);
            hideMessage();
        }
    } catch (error) {
        console.error('Failed to reset stats:', error);
    }
}

// Update stats display
function updateStatsDisplay(stats) {
    document.getElementById('potions-sold').textContent = stats.potions_sold;
    document.getElementById('gold-earned').textContent = stats.gold_earned;
}

// Show purchase message
function showMessage(text) {
    const messageElement = document.getElementById('message');
    messageElement.textContent = text;
    messageElement.style.display = 'block';
}

// Hide purchase message
function hideMessage() {
    const messageElement = document.getElementById('message');
    messageElement.style.display = 'none';
}

// Disable buy buttons
function disableButtons() {
    const buttons = document.querySelectorAll('.buy-button');
    buttons.forEach(button => {
        button.disabled = true;
        button.textContent = 'BUYING...';
    });
}

// Enable buy buttons
function enableButtons() {
    const buttons = document.querySelectorAll('.buy-button');
    buttons.forEach(button => {
        button.disabled = false;
        button.textContent = 'BUY NOW';
    });
}
