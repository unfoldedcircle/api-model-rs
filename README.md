# Unfolded Circle API Models for Rust

This crate provides structs and enums for the [Unfolded Circle Remote Two](https://www.unfoldedcircle.com/) APIs.

The model can be used for user based Rust integration drivers. It is also being used in the Unfolded Circle remote-core
& simulator implementations to make sure it is up-to-date with the API specifications.

## API Specifications

See our [core-api](https://github.com/unfoldedcircle/core-api) GitHub repository.

WebSocket APIs defined with [AsyncAPI](https://www.asyncapi.com/):
- [Integration API](https://github.com/unfoldedcircle/core-api/tree/main/integration-api)
- [Core API](https://github.com/unfoldedcircle/core-simulator/tree/main/core-api)

REST API defined with [OpenAPI](https://www.openapis.org/):
- [Core REST API](https://github.com/unfoldedcircle/core-simulator/tree/main/core-api)

## Versioning

The API is versioned according to [SemVer](https://semver.org/).

The API models will closely follow the core-api versioning. The major version will relate to the
corresponding API release. Not all defined models in the API might be immediately included and might be added later in
a patch or minor release.

**Any major version zero (`0.y.z`) is for initial development and may change at any time!**  
I.e. backward compatibility for minor releases is not yet established, anything MAY change at any time!

## Contributions

Please read our [contribution guidelines](./CONTRIBUTING.md) before opening a pull request.

## License

The API models are licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).  
All graphics copyright © Unfolded Circle ApS 2022.

