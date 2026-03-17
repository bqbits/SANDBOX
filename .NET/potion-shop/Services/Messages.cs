namespace potion_shop.Services
{
    /// <summary>
    /// Fun messages for potion purchases
    /// </summary>
    public static class Messages
    {
        private static readonly string[] PurchaseMessages = new[]
        {
            "You got a Red Potion! Your hearts are refilled!",
            "A fine purchase, brave hero! May it serve you well!",
            "One Red Potion coming right up! Stay safe out there!",
            "Excellent choice! This potion has saved many adventurers!",
            "A wise investment! You'll thank yourself later!",
            "Thank you for your patronage, noble warrior!",
            "It's dangerous to go alone! Take this potion!",
            "May this potion guide you to victory!"
        };

        private static readonly Random _random = new Random();

        public static string GetRandomMessage()
        {
            int index = _random.Next(PurchaseMessages.Length);
            return PurchaseMessages[index];
        }
    }
}
