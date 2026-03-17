# Installation Checklist

Use this checklist to verify your installation is complete and working.

## Prerequisites Check

- [ ] PHP 8.1 or higher installed (`php --version`)
- [ ] Composer installed (`composer --version`)
- [ ] Git installed (optional, for version control)

## Installation Steps

### 1. Initial Setup
- [ ] Navigate to project directory
- [ ] Run `composer install` (or use `./setup.sh`)
- [ ] Copy `.env.example` to `.env`
- [ ] Run `php artisan key:generate`
- [ ] Create `database/database.sqlite` file
- [ ] Set permissions on storage and bootstrap/cache

### 2. Verify File Structure
- [ ] `/app/Http/Controllers/ShopController.php` exists
- [ ] `/resources/views/shop.blade.php` exists
- [ ] `/routes/web.php` exists
- [ ] `/routes/api.php` exists
- [ ] `/.env` file exists (with APP_KEY set)
- [ ] `/storage/framework/sessions` directory exists

### 3. Start Server
- [ ] Run `php artisan serve`
- [ ] Server starts without errors
- [ ] Visit http://localhost:8000

### 4. Test Main Page
- [ ] Shop page loads successfully
- [ ] Banner displays "Benson's Potion Store"
- [ ] Welcome message is visible
- [ ] 3 potion cards are displayed
- [ ] Each card shows "RED POTION"
- [ ] Each card shows "50 GOLD" price
- [ ] Buy buttons are visible
- [ ] Stats footer shows "Total Potions Sold: 0"
- [ ] Stats footer shows "Total Gold Earned: 0"
- [ ] Reset Stats button is visible

### 5. Test Buy Functionality
- [ ] Click a "BUY NOW" button
- [ ] Button changes to "BUYING..."
- [ ] Purchase message appears
- [ ] Stats update to show 1 potion sold
- [ ] Stats update to show 50 gold earned
- [ ] Message disappears after ~4 seconds
- [ ] Click buy again - stats increment correctly
- [ ] Multiple purchases show different random messages

### 6. Test API Endpoints

#### Health Check
- [ ] `curl http://localhost:8000/api/health` returns `{"status":"ok"}`

#### Get Stats
- [ ] `curl http://localhost:8000/api/stats` returns JSON with potions_sold and gold_earned

#### Buy Potion
```bash
curl -X POST http://localhost:8000/api/buy \
  -H "Content-Type: application/json" \
  -H "X-CSRF-TOKEN: $(curl -s http://localhost:8000 | grep csrf-token | sed 's/.*content="\([^"]*\)".*/\1/')"
```
- [ ] Returns JSON with success, message, and stats

#### Reset Stats
```bash
curl -X POST http://localhost:8000/api/reset \
  -H "Content-Type: application/json" \
  -H "X-CSRF-TOKEN: $(curl -s http://localhost:8000 | grep csrf-token | sed 's/.*content="\([^"]*\)".*/\1/')"
```
- [ ] Returns JSON with stats reset to 0

### 7. Test Reset Functionality
- [ ] Buy some potions
- [ ] Stats show non-zero values
- [ ] Click "RESET STATS" button
- [ ] Stats return to 0
- [ ] Message (if visible) disappears

### 8. Test Visual Design
- [ ] Background is green gradient
- [ ] Potion cards have golden borders
- [ ] Cards have sparkle animations in corners
- [ ] Banner has floating animation
- [ ] Gold coins rotate
- [ ] Heart emoji beats
- [ ] Hover over cards - they lift and glow
- [ ] Buy buttons have nice gradient
- [ ] Button hover effect works

### 9. Test Session Persistence
- [ ] Buy some potions
- [ ] Note the stats values
- [ ] Refresh the page (F5 or Cmd+R)
- [ ] Stats remain the same (not reset)
- [ ] Open in new tab - same stats visible
- [ ] Open in different browser - different stats (new session)

### 10. Run Automated Tests (Optional)
- [ ] Run `php artisan test`
- [ ] All tests pass
- [ ] 6 tests should complete successfully

## Troubleshooting

### Issue: Permission Denied
**Solution:**
```bash
chmod -R 775 storage bootstrap/cache
```

### Issue: Application Key Not Set
**Solution:**
```bash
php artisan key:generate
```

### Issue: Sessions Not Working
**Solution:**
```bash
mkdir -p storage/framework/sessions
chmod -R 775 storage
```

### Issue: 404 on API Routes
**Solution:**
Check that `/routes/api.php` exists and contains the route definitions.

### Issue: CSRF Token Mismatch
**Solution:**
Clear browser cookies and refresh the page.

### Issue: Composer Not Found
**Solution:**
Install Composer from https://getcomposer.org/download/

### Issue: PHP Version Too Old
**Solution:**
Upgrade to PHP 8.1 or higher.

## Success Criteria

You should be able to:
1. Load the shop page
2. Buy potions and see stats increment
3. See random purchase messages
4. Reset stats to zero
5. Refresh page and see stats persist
6. All visual animations work smoothly

## Next Steps

Once everything is working:
- [ ] Read the README.md for detailed documentation
- [ ] Check out PROJECT_SUMMARY.md for architecture details
- [ ] Explore the code in `/app/Http/Controllers/ShopController.php`
- [ ] Customize the styling in `/resources/views/shop.blade.php`
- [ ] Add more features (see README for ideas)

## Support

If you encounter issues:
1. Check the Laravel log: `tail -f storage/logs/laravel.log`
2. Check PHP error log
3. Verify all files exist as listed in PROJECT_SUMMARY.md
4. Make sure all prerequisites are installed

Happy potion selling!
