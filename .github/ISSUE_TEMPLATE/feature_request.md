name: Feature Request
description: Suggest an idea for this project
title: "[FEATURE]: "
labels: ["enhancement"]
body:
  - type: markdown
    attributes:
      value: |
        Thanks for suggesting a new feature! Please provide as much detail as possible.

  - type: textarea
    id: problem
    attributes:
      label: Problem Statement
      description: What problem does this feature solve? What limitation are you facing?
      placeholder: Describe the problem you're trying to solve...
    validations:
      required: true

  - type: textarea
    id: solution
    attributes:
      label: Proposed Solution
      description: What would you like to see implemented?
      placeholder: Describe your proposed solution in detail...
    validations:
      required: true

  - type: textarea
    id: alternatives
    attributes:
      label: Alternative Solutions
      description: Have you considered any alternative solutions or workarounds?
      placeholder: Describe any alternative approaches you've considered...

  - type: dropdown
    id: area
    attributes:
      label: Feature Area
      description: Which part of the application does this feature relate to?
      multiple: true
      options:
        - Configuration Parser
        - TUI Interface
        - Export/Import
        - KWin Integration
        - CLI Interface
        - Documentation
        - Other
    validations:
      required: true

  - type: dropdown
    id: priority
    attributes:
      label: Priority
      description: How important is this feature to you?
      options:
        - Low - Nice to have
        - Medium - Would improve workflow
        - High - Essential for my use case
        - Critical - Blocking current usage
    validations:
      required: true

  - type: textarea
    id: implementation
    attributes:
      label: Implementation Ideas
      description: Any thoughts on how this could be implemented?
      placeholder: Technical details, API ideas, etc...

  - type: textarea
    id: mockups
    attributes:
      label: Mockups or Examples
      description: Any mockups, screenshots, or examples to illustrate the feature
      placeholder: You can attach images or link to examples...

  - type: textarea
    id: additional
    attributes:
      label: Additional Context
      description: Any other relevant information
      placeholder: Additional context about the feature request...

  - type: checkboxes
    id: terms
    attributes:
      label: Confirmation
      description: Please confirm the following
      options:
        - label: I have searched existing issues for similar feature requests
          required: true
        - label: I understand this is a volunteer project and features are implemented when possible
          required: true
        - label: I'm willing to help implement this feature (if applicable)
          required: false