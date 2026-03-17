namespace potion_shop_k8s.Services
{
    /// <summary>
    /// Service for generating fun purchase messages
    /// </summary>
    public class MessageService
    {
        private readonly string[] _purchaseMessages = new[]
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

        /// <summary>
        /// Get a random purchase message
        /// </summary>
        public string GetRandomMessage()
        {
            var random = new Random();
            return _purchaseMessages[random.Next(_purchaseMessages.Length)];
        }
    }
}
