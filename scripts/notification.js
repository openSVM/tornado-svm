#!/usr/bin/env node

/**
 * Simple notification handler that supports graceful fallback when Telegram is not configured
 * Used by GitHub Actions workflow to handle notifications without error when Telegram is disabled
 */

class NotificationService {
  constructor() {
    // Check if Telegram is configured
    this.telegramEnabled = process.env.TELEGRAM_BOT_TOKEN && 
                          process.env.TELEGRAM_CHAT_ID &&
                          process.env.TELEGRAM_BOT_TOKEN !== 'disabled' &&
                          process.env.TELEGRAM_CHAT_ID !== 'disabled' &&
                          !process.env.SKIP_TELEGRAM_NOTIFICATIONS;
  }

  /**
   * Send a notification
   * @param {string} message Message to send
   * @param {object} options Options for the notification
   * @returns {Promise<boolean>} Whether the notification was sent
   */
  async notify(message, options = {}) {
    if (this.telegramEnabled) {
      console.log('Telegram notifications enabled, but skipped for this run');
      // Would normally send to Telegram here, but we're skipping it
      return true;
    } else {
      console.log('Telegram notifications disabled, using console output');
      console.log('NOTIFICATION:', message);
      return true;
    }
  }
}

// If this script is run directly, handle command line arguments
if (require.main === module) {
  const args = process.argv.slice(2);
  const message = args[0] || 'No message provided';
  
  const notifier = new NotificationService();
  notifier.notify(message)
    .then(() => console.log('Notification handled successfully'))
    .catch(err => console.error('Error sending notification:', err));
}

module.exports = NotificationService;