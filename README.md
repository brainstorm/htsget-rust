# htsget

This is just an autogenerated rust client/server OpenAPI scaffold for [htsget](https://github.com/samtools/hts-specs/pull/385):

`openapi-generator generate -i spec/htsget-openapi.yaml -g rust -o src/htsget-client`
`openapi-generator generate -i spec/htsget-openapi.yaml -g rust-server -o src/htsget-server`

Here be dragons. I just got scared about the amount of code generated by it and going for [serverless-aws-rust-http](https://github.com/softprops/serverless-aws-rust-http) instead, at the time of writing this.