// Fetch and display stats on page load
document.addEventListener('DOMContentLoaded', function() {
    fetchStats();
});

// Fetch shop statistics
async function fetchStats() {
    try {
        const response = await fetch('/api/stats');
        const data = await response.json();
        updateStatsDisplay(data.potions_sold, data.gold_earned);
    } catch (error) {
        console.error('Failed to fetch stats:', error);
    }
}

// Buy a potion
async function buyPotion() {
    // Disable all buy buttons
    disableButtons(true);

    try {
        const response = await fetch('/api/buy', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });
        const data = await response.json();

        if (data.success) {
            // Display message
            showMessage(data.message);

            // Update stats
            updateStatsDisplay(data.stats.potions_sold, data.stats.gold_earned);
        }
    } catch (error) {
        console.error('Failed to buy potion:', error);
    } finally {
        // Re-enable buy buttons
        disableButtons(false);
    }
}

// Reset statistics
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
            // Update stats
            updateStatsDisplay(data.stats.potions_sold, data.stats.gold_earned);

            // Hide message if visible
            hideMessage();
        }
    } catch (error) {
        console.error('Failed to reset stats:', error);
    }
}

// Update stats display
function updateStatsDisplay(potionsSold, goldEarned) {
    document.getElementById('potions-sold').textContent = potionsSold;
    document.getElementById('gold-earned').textContent = goldEarned;
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

// Disable/enable buy buttons
function disableButtons(disabled) {
    const buttons = document.querySelectorAll('.buy-button');
    buttons.forEach(button => {
        button.disabled = disabled;
        button.textContent = disabled ? 'BUYING...' : 'BUY NOW';
    });
}
