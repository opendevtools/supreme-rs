{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    ["@semantic-release/exec", {
      "verifyConditionsCmd": "semantic-release-rust verify-conditions",
      "prepareCmd": "semantic-release-rust prepare ${nextRelease.version}",
    }],
    ["@semantic-release/git", {
      "assets": ["Cargo.toml", "Cargo.lock", "CHANGELOG.md"]
    }]
  ]
}

