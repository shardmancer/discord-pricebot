# ✨♣️🎰 Discord PriceBot 💎📊

A high-performance **Rust-powered Discord bot** that **updates channel names with live cryptocurrency prices** and **sends real-time price alerts via webhooks**. Built for **crypto traders, Web3 communities, and Discord servers** seeking **24/7 price tracking** with precision and style.

## 💎 Features
- 📈 **Live Price Updates** – BTC, TON, SOL, BNB, ETH (every 5 minutes)
- 💬 **Channel Name Ticker** – Fullwidth Unicode pricing for a clean display
- 🛸 **Webhook Price Alerts** – Instant price notifications to Discord
- 🤖 **Rust-Powered** – Memory-safe, fast, built for uptime
- 💡 **Raspberry Pi & Linux Optimized** – Runs as a systemd service
- 🌍 **Free Price API** via CoinPaprika

## 🛠️ Requirements
- 5 Discord text channels (BTC, TON, SOL, BNB, ETH)
- Discord Webhooks (1 per channel)
- Discord Bot with permissions to **manage channels & send messages**

## 💪 Setup
1. **Clone the Repo:**
   ```bash
   git clone https://github.com/shardmancer/discord-pricebot.git
   cd discord-pricebot
   ```

2. **Create a `.env` file** (refer to `.env.example` for variables):
   ```dotenv
   DISCORD_BOT_TOKEN=YOUR_DISCORD_BOT_TOKEN
   BTC_CHANNEL_ID=YOUR_BTC_CHANNEL_ID
   TON_CHANNEL_ID=YOUR_TON_CHANNEL_ID
   SOL_CHANNEL_ID=YOUR_SOL_CHANNEL_ID
   BNB_CHANNEL_ID=YOUR_BNB_CHANNEL_ID
   ETH_CHANNEL_ID=YOUR_ETH_CHANNEL_ID
   BTC_WEBHOOK=YOUR_BTC_WEBHOOK
   TON_WEBHOOK=YOUR_TON_WEBHOOK
   SOL_WEBHOOK=YOUR_SOL_WEBHOOK
   BNB_WEBHOOK=YOUR_BNB_WEBHOOK
   ETH_WEBHOOK=YOUR_ETH_WEBHOOK
   ```

3. **Build & Run:**
   ```bash
   cargo build --release
   ./target/release/pricebot
   ```

4. **Optional: Set Up as systemd Service for 24/7 Uptime:**
   ```bash
   sudo nano /etc/systemd/system/pricebot.service
   ```
   Example:
   ```ini
   [Unit]
   Description=Discord PriceBot Service
   After=network.target

   [Service]
   User=YOUR_USER
   WorkingDirectory=/path/to/discord-pricebot
   ExecStart=/path/to/discord-pricebot/target/release/pricebot
   Restart=always
   Environment="RUST_BACKTRACE=1"

   [Install]
   WantedBy=multi-user.target
   ```
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable pricebot.service
   sudo systemctl start pricebot.service
   ```

## 🌟 Built With
- 🤖 **Rust** – High-performance, memory-safe
- 📚 **Serenity** – Discord API wrapper
- 🌐 **CoinPaprika API** – Free price data
- 🛠️ **Systemd** – Uptime stability on Linux

## 💎 Designed for
- 📈 Crypto Traders & Signal Groups
- 🌟 Web3 & NFT Communities
- 👨‍💼 Tech Projects & DeFi Servers

## 💡 Tags
`rust discord-bot cryptocurrency crypto-price price-bot discord-price-bot price-ticker serenity-bot raspberry-pi systemd web3 defi nft`

**By @shardmancer** ✨♣️🎰