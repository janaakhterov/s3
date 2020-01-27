###TODO:
  - Add support for query parameters in requests
  - Add support for creating client from default config options
    - Read config file ~/.aws/config and update hyper::client to use those settings
  - Implement more request types
  - Add CI
  - Add example to front page of docs

###FIXME:
  - Allow to config request to put bucket in path, or in domain
    - 'http://localhost:9000/bucket' vs 'http://bucket.localhost:9000'
    - This is required because cannot use a subdomain on localhost, but
    - that should be the perferred way.
    - Perhaps look for `localhost` inside domain
