# Wings Publishers

Wings publishers is a canister that defines endpoints responsible for handling request to write, request review, approve, and publish articles on the IC protocol.

The canister can be tested locally as defined in the instructions below.

## :package: Built With

    - rust
    - dfx
    - Ic protocol

## :computer: Getting Started

    To get a local copy up and running follow these simple steps.

## :arrow_heading_down: Install

1. Clone the repository to your local machine

```sh
$ git clone git@github.com:kiberuJ/Wings-Publishers.git
```

2. cd into the directory

```sh
$ cd Wings-Publishers
```

3. Initialize the local Internet Computer

```sh
dfx start --background
```

4. Register, build, and deploy canister on the local Internet Computer

```sh
npm run gen-deploy
```

## :arrow_forward: Usage

After successful deployement on the local Internet Computer, one can be able to interact with the canister using the terminal by invoking `dfx call canisterName methodName(params)` commands or through the provided candid interface.

## :busts_in_silhouette: Authors

👤 **Jane Kiberu**

- Github: [@KiberuJ](https://github.com/kiberuJ)

## 🤝 Contributing

    Contributions, issues and feature requests are welcome!

Feel free to check the [issues page](../../issues).

## :star2: Show your support

    Give a ⭐️ if you like this project!
