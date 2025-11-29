# Personal Website with Rust

## 🏗️ Architecture Overview

This project implements a personal website built with Rust using Actix-web. The site features a fox-themed design with a responsive layout and includes Last.fm music integration.

### Technology Stack

- **Backend**: Rust with Actix-web
- **Templating**: Askama (compile-time HTML templates)
- **Styling**: Custom CSS with CSS variables for theming
- **APIs**: GitHub repos & Last.fm music integration
- **Deployment**: Cloudflare Pages with Functions

## 🚀 Getting Started

### Prerequisites

- Rust (1.70.0 or higher)
- Cargo (Rust package manager)
- Git
- Node.js (for Cloudflare deployment)

### Environment Setup

1. Clone the repository:
```bash
git clone https://github.com/System-End/My-website
cd My-website
```

2. Create a `.env` file in the root directory:
```env
HOST=127.0.0.1
PORT=8080
GITHUB_USERNAME=your_github_username
LASTFM_API_KEY=your_lastfm_api_key
LASTFM_USERNAME=your_lastfm_username
```

3. Build and run locally:
```bash
# Development
cargo run

# Release build
cargo build --release
./target/release/personal-site
```

## 💻 Local Development

Start the development server:
```bash
cargo run
```

The application will be available at http://localhost:8080

### Project Structure

```
.
├── Cargo.toml          # Rust dependencies
├── src/
│   ├── main.rs         # Application entry point and routes
│   ├── templates.rs    # Template definitions
│   ├── bin/
│   │   └── build.rs    # Static site generator for Cloudflare
│   └── api/
│       ├── mod.rs      # API module exports
│       ├── github.rs   # GitHub repos API
│       ├── lastfm.rs   # Last.fm music API
│       └── health.rs   # Health check endpoint
├── templates/          # HTML templates
│   ├── base.html       # Base layout
│   ├── about.html      # About page
│   ├── projects.html   # Projects page
│   └── 404.html        # Not found page
├── static/             # Static assets
│   └── styles.css      # CSS styles
├── public/             # Public assets (images, logos)
├── functions/          # Cloudflare Pages Functions
│   └── api/
│       ├── lastfm.js   # Last.fm API endpoint
│       ├── repos.js    # GitHub repos endpoint
│       └── health.js   # Health check endpoint
└── wrangler.toml       # Cloudflare configuration
```

## 🌐 API Endpoints

- `GET /` - About page
- `GET /projects` - Projects page
- `GET /api/repos` - Fetch GitHub repositories
- `GET /api/lastfm` - Fetch current Last.fm track
- `GET /api/health` - Health check

## ☁️ Cloudflare Deployment

### Build Static Site

```bash
# Generate static HTML files
cargo run --bin static-gen
```

This creates a `dist/` directory with all static files.

### Deploy to Cloudflare Pages

1. Install Wrangler CLI:
```bash
npm install -g wrangler
```

2. Login to Cloudflare:
```bash
wrangler login
```

3. Set up secrets:
```bash
npx wrangler pages secret put LASTFM_API_KEY
npx wrangler pages secret put LASTFM_USERNAME
```

4. Deploy:
```bash
npx wrangler pages deploy dist
```

### GitHub Actions Deployment

The repository includes automatic deployment via GitHub Actions. Set these secrets in your repository:
- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`
- `LASTFM_API_KEY`
- `LASTFM_USERNAME`

## 🎵 Last.fm Integration

The site displays your currently playing track from Last.fm. Features:
- Real-time "Now Playing" indicator
- Album art display
- Falls back to Spotify playlist embed when not playing

Get your Last.fm API key at: https://www.last.fm/api/account/create

## 🐛 Troubleshooting

### Common Issues

1. Build Issues:
```bash
# Clean and rebuild
cargo clean
cargo build
```

2. Port already in use:
```bash
# Change port in .env file
PORT=3000
```

3. Last.fm not showing:
- Verify your API key is correct
- Check that your Last.fm username is correct
- Make sure you have recent scrobbles

## 📚 Additional Resources

- [Actix-web Documentation](https://actix.rs/docs/)
- [Askama Templates](https://djc.github.io/askama/)
- [Cloudflare Pages](https://developers.cloudflare.com/pages/)
- [Last.fm API](https://www.last.fm/api)
- [Rust Documentation](https://doc.rust-lang.org/)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit changes
4. Push to the branch
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Legacy React Version

The original React/TypeScript version of the site can be found in the `old-frontend-src` directory for reference.