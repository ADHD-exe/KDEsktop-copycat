name: Bug Report
description: File a bug report to help us improve
title: "[BUG]: "
labels: ["bug"]
body:
  - type: markdown
    attributes:
      value: |
        Thanks for taking the time to fill out this bug report! Please provide as much detail as possible.

  - type: textarea
    id: description
    attributes:
      label: Bug Description
      description: A clear and concise description of what the bug is
      placeholder: Describe what happened...
    validations:
      required: true

  - type: textarea
    id: reproduction
    attributes:
      label: Steps to Reproduce
      description: Steps to reproduce the behavior
      placeholder: |
        1. Run 'kdesktop-copycat tui'
        2. Navigate to '...'
        3. Press '...'
        4. See error
    validations:
      required: true

  - type: textarea
    id: expected
    attributes:
      label: Expected Behavior
      description: What you expected to happen
      placeholder: Describe what should have happened...
    validations:
      required: true

  - type: textarea
    id: actual
    attributes:
      label: Actual Behavior
      description: What actually happened
      placeholder: Describe what actually happened...
    validations:
      required: true

  - type: textarea
    id: environment
    attributes:
      label: Environment Information
      description: Information about your system
      placeholder: |
        - OS: [e.g. Ubuntu 22.04, Fedora 38, Arch Linux]
        - KDE Plasma Version: [e.g. 6.0.4, 6.1.0]
        - Rust Version: [e.g. 1.75.0]
        - KDEsktop-copycat Version: [e.g. 0.1.0]
    validations:
      required: true

  - type: textarea
    id: logs
    attributes:
      label: Logs and Error Messages
      description: Paste any relevant logs or error messages
      render: shell
      placeholder: Paste logs here, use ```bash ... ``` for formatting

  - type: textarea
    id: additional
    attributes:
      label: Additional Context
      description: Any additional information about the problem
      placeholder: Screenshots, configuration files, etc.

  - type: checkboxes
    id: terms
    attributes:
      label: Confirmation
      description: Please confirm the following
      options:
        - label: I have searched existing issues for similar bugs
          required: true
        - label: I have provided all the requested information
          required: true
        - label: This is not a security vulnerability (use security@example.com for security issues)
          required: true