#!/bin/bash

# Test script for Benson's Potion Shop API
# Make sure the server is running before executing this script

echo "Testing Benson's Potion Shop API..."
echo "===================================="
echo ""

# Test health endpoint
echo "1. Testing Health Check (GET /api/health)"
curl -s http://localhost:8080/api/health | jq .
echo ""
echo ""

# Test initial stats
echo "2. Getting Initial Stats (GET /api/stats)"
curl -s http://localhost:8080/api/stats | jq .
echo ""
echo ""

# Buy a potion
echo "3. Buying a Potion (POST /api/buy)"
curl -s -X POST http://localhost:8080/api/buy | jq .
echo ""
echo ""

# Buy another potion
echo "4. Buying Another Potion (POST /api/buy)"
curl -s -X POST http://localhost:8080/api/buy | jq .
echo ""
echo ""

# Check stats after purchases
echo "5. Getting Stats After Purchases (GET /api/stats)"
curl -s http://localhost:8080/api/stats | jq .
echo ""
echo ""

# Reset stats
echo "6. Resetting Stats (POST /api/reset)"
curl -s -X POST http://localhost:8080/api/reset | jq .
echo ""
echo ""

# Check stats after reset
echo "7. Getting Stats After Reset (GET /api/stats)"
curl -s http://localhost:8080/api/stats | jq .
echo ""
echo ""

echo "===================================="
echo "API Testing Complete!"
echo ""
echo "Now open http://localhost:8080 in your browser to see the shop!"
