# Contributing to NullScript 🎭

Thank you for your interest in contributing to NullScript! We welcome contributions from the community and appreciate your help in making NullScript better.

## 🤝 How to Contribute

### Types of Contributions

We welcome various types of contributions:

- **Bug Reports**: Found a bug? Let us know!
- **Feature Requests**: Have an idea for a new feature?
- **Code Contributions**: Fix bugs, add features, improve documentation
- **Documentation**: Improve README, add examples, write guides
- **Testing**: Test the transpiler with different scenarios
- **Examples**: Create interesting example programs

### Getting Started

1. **Fork the repository**
2. **Clone your fork**:
   ```bash
   git clone https://github.com/nullscript-lang/nullscript.git
   cd nullscript
   ```
3. **Install dependencies**:
   ```bash
   npm install
   ```
4. **Build the project**:
   ```bash
   npm run build
   ```

### Development Workflow

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. **Make your changes**
3. **Test your changes**:
   ```bash
   npm run test:examples
   npm run test:run
   ```
4. **Commit your changes**:
   ```bash
   git commit -m "feat: add new keyword alias"
   ```
5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
6. **Create a Pull Request**

## 🐛 Bug Reports

When reporting bugs, please include:

- **Description**: What happened?
- **Steps to reproduce**: How can we reproduce this?
- **Expected behavior**: What should happen?
- **Actual behavior**: What actually happened?
- **Environment**: OS, Node.js version, NullScript version
- **Example code**: Minimal code that reproduces the issue

## 💡 Feature Requests

When requesting features, please include:

- **Description**: What feature would you like?
- **Use case**: Why is this feature needed?
- **Proposed implementation**: How should it work?
- **Examples**: Show how it would be used

## 📝 Code Style

### TypeScript/JavaScript

- Use TypeScript for all new code
- Follow the existing code style
- Add proper type annotations
- Include JSDoc comments for public APIs
- Use meaningful variable and function names

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Pull Request Guidelines

- **Title**: Clear, descriptive title
- **Description**: Explain what the PR does and why
- **Tests**: Include tests for new features
- **Documentation**: Update docs if needed
- **Examples**: Add examples for new features

## 🧪 Testing

### Running Tests

```bash
# Test example transpilation
npm run test:examples

# Run example programs
npm run test:run

# Build the project
npm run build
```

### Adding Tests

When adding new features or fixing bugs:

1. **Create test examples** in the `examples/` directory
2. **Test transpilation** with `npm run test:examples`
3. **Test execution** with `npm run test:run`
4. **Test edge cases** and error conditions

## 📚 Documentation

### Documentation Standards

- **README.md**: Main project documentation
- **Examples**: Clear, well-commented example programs
- **Code comments**: Explain complex logic
- **API documentation**: Document public APIs

### Adding Documentation

- Update README.md for new features
- Add examples to the `examples/` directory
- Include usage examples in code comments
- Update language reference if adding new keywords

## 🎯 Project Goals

### NullScript Philosophy

- **Fun but functional**: Playful syntax that actually works
- **TypeScript compatibility**: 100% compatible with TypeScript
- **Educational**: Help people understand transpilers
- **Production ready**: Actually usable in real projects

### Guidelines for Contributions

- **Maintain compatibility**: Don't break TypeScript compatibility
- **Keep it fun**: New keywords should be playful but clear
- **Document everything**: New features need documentation
- **Test thoroughly**: Ensure everything works correctly

## 🚀 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **Major**: Breaking changes
- **Minor**: New features (backward compatible)
- **Patch**: Bug fixes (backward compatible)

### Release Checklist

Before releasing:

- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Examples work correctly
- [ ] Version is updated in package.json
- [ ] CHANGELOG.md is updated
- [ ] Build is successful

## 📞 Getting Help

### Questions?

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Documentation**: Check the README and examples first

### Community Guidelines

- **Be respectful**: Treat everyone with respect
- **Be helpful**: Help others learn and contribute
- **Be patient**: Everyone learns at their own pace
- **Have fun**: Remember, NullScript is about having fun with code!

## 🎉 Recognition

Contributors will be recognized in:

- **README.md**: List of contributors
- **Release notes**: Credit for contributions
- **GitHub**: Contributor statistics

---

Thank you for contributing to NullScript! 🎭

**Let's make programming fun together!** ❤️
