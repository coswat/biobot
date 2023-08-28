## Telgeram Bio Bot

Highly custamisable bio bot using Rust, ( teloxide x tokio ) specially designed for programmers

[Demo Bot](https://telegram.dog/coswatbot)

Key features

- Custamisable Buttons
- Custamisable Responses
- Custom sponsed button
- Integrated with Telegram Payments

## Setup

Lets checkout the env variables first

```.env
# Bot token
TELOXIDE_TOKEN=

# Port, ex: 3000
PORT=3000

# Host, ( if heroku, then yourapp.herokuapp.com )
HOST=

# Devlopment mode ( set false while production )

DEV=true

# Payment token from @BotFather

PAYMENT_TOKEN=xxx
```

After setuping these you can test this by, 

```bash
cargo build --release
```

then 

```bash
cargo run --release
```

![Bio Bot](https://github-production-user-asset-6210df.s3.amazonaws.com/97345827/263542767-4d1e78a0-315a-4f1c-a94c-20a0416c24ae.jpg)