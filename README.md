# Cross-platform graph visualizer POC monorepo

**graphviz-poc** is a cross-platform proof of concept (POC) for visualizing the **Intuition knowledge graph** or segments of it. This repository provides tools and applications to create detailed visualizations, enabling users to explore, interact with, and understand the relationships within the Intuition knowledge graph. Built with cross-platform support in mind, this monorepo includes components for web, mobile, and desktop environments.

## Overview

The Intuition knowledge graph visualizer allows you to:
- **Explore Knowledge Graphs**: Visualize entire knowledge graphs or specific segments, revealing relationships and insights.
- **Cross-Platform Compatibility**: Access visualizations across various platforms, including web, desktop, and mobile.
- **Interact with Graph Elements**: View details and relationships within nodes, making it easy to explore Intuition's decentralized knowledge network.

## Diagram

Below is the structure and current state of the **graphviz-poc** monorepo:

[Diagram](https://whimsical.com/graph-view-proposal-3AMs2D5RQW87p2cu1sTGpk)

## Structure

- [crates](./crates/) - rust crates
  - [bevy-graph-view](./crates/bevy-graph-view/) - the main graph view 
- [packages](./packages/) - npm packages
  - [wasm-graph-view](/packages/wasm-graph-view/) 
- [web](./web/) - intuition starter template (todo)
- [web-vite](./web-vite/) - vanilla vite website
- [mobile-rn](./mobile-rn/) - react native mobile app (todo)
- [mobile-custom-rn](./mobile-custom-rn/) - custom react native mobile app (todo, broken)
- [mobile](./mobile/) - ios mobile app 

## Contributing

We welcome contributions to improve the graphviz-poc project! Here's how you can help:

### Bug Reports & Feature Requests
- Open an issue describing the bug or feature request
- Include steps to reproduce for bugs
- Provide context and use cases for feature requests

### Pull Requests
1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

### Development Guidelines
- Follow existing code style and conventions
- Add tests for new features
- Update documentation as needed
- Keep pull requests focused and atomic

We appreciate all contributions, from code to documentation improvements!


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This project is a proof of concept (POC) and is provided "as is" without warranty of any kind, express or implied. The authors and contributors are not responsible for any damages or liabilities arising from the use of this software.

The Intuition knowledge graph visualizer POC is not intended for production use and may contain bugs, incomplete features, or experimental functionality.

## Support

For questions, bug reports, or feature requests, please open an issue in the GitHub repository.

Note that as this is a POC, support is provided on a best-effort basis by the community and maintainers.
