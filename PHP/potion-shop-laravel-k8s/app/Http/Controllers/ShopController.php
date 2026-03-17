<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Session;

class ShopController extends Controller
{
    /**
     * Purchase messages for random selection
     */
    private const PURCHASE_MESSAGES = [
        "You got a Red Potion! Your hearts are refilled!",
        "A fine purchase, brave hero! May it serve you well!",
        "One Red Potion coming right up! Stay safe out there!",
        "Excellent choice! This potion has saved many adventurers!",
        "A wise investment! You'll thank yourself later!",
        "Thank you for your patronage, noble warrior!",
        "It's dangerous to go alone! Take this potion!",
        "May this potion guide you to victory!",
    ];

    /**
     * Display the shop page
     */
    public function index()
    {
        return view('shop');
    }

    /**
     * Purchase a potion
     */
    public function buy(Request $request)
    {
        // Get current stats from session (default to 0)
        $potionsSold = Session::get('potions_sold', 0);
        $goldEarned = Session::get('gold_earned', 0);

        // Increment stats
        $potionsSold++;
        $goldEarned += 50;

        // Save to session
        Session::put('potions_sold', $potionsSold);
        Session::put('gold_earned', $goldEarned);

        // Get random message
        $message = self::PURCHASE_MESSAGES[array_rand(self::PURCHASE_MESSAGES)];

        return response()->json([
            'success' => true,
            'message' => $message,
            'stats' => [
                'potions_sold' => $potionsSold,
                'gold_earned' => $goldEarned,
            ],
        ]);
    }

    /**
     * Get shop statistics
     */
    public function stats(Request $request)
    {
        return response()->json([
            'potions_sold' => Session::get('potions_sold', 0),
            'gold_earned' => Session::get('gold_earned', 0),
        ]);
    }

    /**
     * Reset shop statistics
     */
    public function reset(Request $request)
    {
        Session::put('potions_sold', 0);
        Session::put('gold_earned', 0);

        return response()->json([
            'success' => true,
            'stats' => [
                'potions_sold' => 0,
                'gold_earned' => 0,
            ],
        ]);
    }

    /**
     * Health check endpoint
     */
    public function health()
    {
        return response()->json([
            'status' => 'ok',
        ]);
    }
}
