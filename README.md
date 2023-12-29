# Wings Publishers

Wings publishers is a simple Rust canister that implements functionality for managing users and articles, allowing operations such as adding, updating, and deleting users, as well as creating articles, requesting reviews, and approving articles.

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
$ git clone https://github.com/kiberuJ/Wings-Publishers.git
```

2. cd into the directory

```sh
$ cd Wings-Publishers
```

3. Install dependencies
```sh
$ npm install
```
Make sure your dev environment is intialized. Find the initialization process [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/dev-env)

4. Start the local replica of Internet Computer

```sh
dfx start --background
```

4. Register, build, and deploy canister on the local Internet Computer

```sh
npm run gen-deploy
```

## :arrow_forward: Usage

After successful deployement on the local Internet Computer, one can be able to interact with the canister using the terminal by invoking `dfx call wings_publishers_backend methodName(params)` commands or through the provided candid interface.

## :busts_in_silhouette: Authors

👤 **Jane Kiberu**

- Github: [@KiberuJ](https://github.com/kiberuJ)

## 🤝 Contributing

    Contributions, issues and feature requests are welcome!

Feel free to check the [issues page](../../issues).

## :star2: Show your support

    Give a ⭐️ if you like this project!
