# Personal Website with Rust

## 🏗️ Architecture Overview

This project implements a personal website built with Rust using Actix-web. The site features a fox-themed design with a responsive layout.

### Technology Stack

- **Backend**: Rust with Actix-web
- **Templating**: Askama (compile-time HTML templates)
- **Styling**: Custom CSS with CSS variables for theming
- **API**: GitHub repos integration

## 🚀 Getting Started

### Prerequisites

- Rust (1.70.0 or higher)
- Cargo (Rust package manager)
- Git

### Environment Setup

1. Clone the repository:
```bash
git clone https://github.com/System-End/My-website
cd My-website
```

2. Create a `.env` file in the root directory (optional):
```env
HOST=127.0.0.1
PORT=8080
GITHUB_USERNAME=your_github_username
```

3. Build and run:
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
│   └── api/
│       ├── mod.rs      # API module exports
│       ├── github.rs   # GitHub repos API
│       └── health.rs   # Health check endpoint
├── templates/          # HTML templates
│   ├── base.html       # Base layout
│   ├── about.html      # About page
│   ├── projects.html   # Projects page
│   └── 404.html        # Not found page
├── static/             # Static assets
│   └── styles.css      # CSS styles
└── public/             # Public assets (images, logos)
```

## 🌐 API Endpoints

- `GET /` - About page
- `GET /projects` - Projects page
- `GET /api/repos` - Fetch GitHub repositories
- `GET /api/health` - Health check

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

## 📚 Additional Resources

- [Actix-web Documentation](https://actix.rs/docs/)
- [Askama Templates](https://djc.github.io/askama/)
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