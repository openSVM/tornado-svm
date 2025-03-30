#!/usr/bin/env node

/**
 * Simple notification handler that uses console output
 * Used by GitHub Actions workflow to output status messages without external dependencies
 */

class NotificationService {
  /**
   * Send a notification via console output
   * @param {string} message Message to send
   * @param {object} options Options for the notification (unused)
   * @returns {Promise<boolean>} Always returns true
   */
  async notify(message, options = {}) {
    console.log('🔔 NOTIFICATION:', message);
    return true;
  }
}

// If this script is run directly, handle command line arguments
if (require.main === module) {
  const args = process.argv.slice(2);
  const message = args[0] || 'No message provided';
  
  const notifier = new NotificationService();
  notifier.notify(message)
    .then(() => console.log('Notification logged successfully'))
    .catch(err => console.error('Error logging notification:', err));
}

module.exports = NotificationService;
