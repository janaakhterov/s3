###TODO:
  - Add support for query parameters in requests
  - Add support for creating client from default config options
    - From file using `AWS_SHARED_CREDENTIALS_FILE` to find file, otherwise ~/.aws~/.aws/credentials
      - Requires implementing a parser for the file. Perhaps `nom`?
  - Implement more request types
  - Add CI
  - Add example to front page of docs
