# Contributing to Secure Messaging App (sec_mes)

Thank you for your interest in contributing to the Secure Messaging App. Contributions are welcome.

## How to Contribute

1. **Fork the Repository**

2. **Clone the Repository**:

    ```bash
    git clone https://github.com/gituser12981u2/sec_msg/tree/master
    ```

3. **Create a Branch**:

    ```bash
    git checkout -b feature/feature-name
    ```

4. **Set up Development Environment**: Use a standardized environment with docker

    **For VS Code Users**

    1. Install the "Dev Containers" extension in VS Code.
    2. When prompted, click "Reopen in Container".
    3. VS Code will build the Docker container and set up the development environment.

    **For Non-VS code Users**

    1. Ensure you have Docker installed on your system.
    2. Run `docker-compose build` to build the Docker image.
    3. Run `docker-compose run --rm dev` to start a shell in the development container.

    The development container includes all necessary tools and git hooks for development.

    **Git configuration**

    The container will attempt to port the local machines git configuration. Please do the following:

    - Create a .env file in the project root with these variables

    If no .env file is created, then the container will prompt to enter a git user name and email when the container starts or is rebuilt.

    **GPG Signing for Commits**

    On first run, the container will generate a new project specific GPG key. This key needs to be added to ones Github account:

    1. After the container starts, the terminal will show a message with the GPG public key
    2. Copy the entire key, including the `-----BEGIN PGP PUBLIC KEY BLOCK-----` and `-----END PGP PUBLIC KEY BLOCK-----` lines.
    3. Go to GitHub account settings, navigate to "SSH and GPG keys", and add a new GPG key with the copied content.

    The key will persist between container rebuilds, so this has to only be done once per machine.

5. **Make Changes**: Implement the changes in the codebase.
6. **Run Tests**: Ensure all tests pass before submitting the changes.

    ```bash
    cargo test
    ```

7. **Commit Changes**: Follow the commit message guidelines.

    ```bash
    git commit -m "feat: Add new feature"
    ```

8. **Push to Fork**:

    ```bash
    git push origin feature/feature-name
    ```

9. **Create a Pull Request**: Open a pull request from the forked repository to the main repository.

## Commit Message Guidelines

[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) are used for the commit messages in this project.

*feat: A new feature.
*fix: A bug fix.
*docs: Documentation only changes.
*style: Changes that do no affect the meaning of the code (white-space, formatting, missing semi-colons, etc).
*refactor: A code change that neither fixes a bug nor adds a feature.
*test: Adding missing or correcting existing tests.
*chore: Changes to the build process or auxiliary tools and libraries such as documentation generation.

## Contribution Requirements

- **Signed Commits**: All commits must be signed.
- **Linear History**: Ensure a linear commit history by rebasing instead of merging.
- **Status Checks**: All status checks must pass before a pull request is merged

## License

By contributing, one agree's that contributions will be licensed under the MIT License.

Thank you for your contributions!
