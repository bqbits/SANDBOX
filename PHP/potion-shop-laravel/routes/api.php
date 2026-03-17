<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\ShopController;

/*
|--------------------------------------------------------------------------
| API Routes
|--------------------------------------------------------------------------
|
| Here is where you can register API routes for your application. These
| routes are loaded by the RouteServiceProvider and all of them will
| be assigned to the "api" middleware group. Make something great!
|
*/

Route::post('/buy', [ShopController::class, 'buy'])->name('api.buy');
Route::get('/stats', [ShopController::class, 'stats'])->name('api.stats');
Route::post('/reset', [ShopController::class, 'reset'])->name('api.reset');
Route::get('/health', [ShopController::class, 'health'])->name('api.health');
