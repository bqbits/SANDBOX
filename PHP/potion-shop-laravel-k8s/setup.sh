#!/bin/bash

echo "Setting up Benson's Potion Store..."
echo ""

# Check if composer is installed
if ! command -v composer &> /dev/null
then
    echo "Error: Composer is not installed. Please install Composer first."
    echo "Visit: https://getcomposer.org/download/"
    exit 1
fi

# Check if PHP is installed
if ! command -v php &> /dev/null
then
    echo "Error: PHP is not installed. Please install PHP 8.1 or higher."
    exit 1
fi

# Check PHP version
PHP_VERSION=$(php -r "echo PHP_VERSION;")
echo "PHP version: $PHP_VERSION"

# Install dependencies
echo ""
echo "Installing Composer dependencies..."
composer install

# Copy .env file
if [ ! -f .env ]; then
    echo ""
    echo "Creating .env file..."
    cp .env.example .env
fi

# Generate application key
echo ""
echo "Generating application key..."
php artisan key:generate

# Create SQLite database
echo ""
echo "Creating SQLite database..."
touch database/database.sqlite

# Set permissions
echo ""
echo "Setting permissions..."
chmod -R 775 storage bootstrap/cache

echo ""
echo "Setup complete! You can now run the application with:"
echo "  php artisan serve"
echo ""
echo "Then visit: http://localhost:8000"
