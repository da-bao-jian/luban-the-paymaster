# Web3Auth Integration with UserOp.js Using Next.js

This demonstration application illustrates how to integrate self-custodial social authentication with [Web3Auth](https://web3auth.io/) and [userop.js](https://github.com/stackup-wallet/userop.js).

## Getting Started

Clone this repository to your local machine.

### Prerequisites

- Node.js installed on your system.

- An account with Web3Auth ([https://web3auth.io/](https://web3auth.io/)) to get a client ID token.

- An account with Stackup ([https://app.stackup.sh/](https://app.stackup.sh/)) to get a RPC URL.

### Configuration

The application uses environment variables for configuration.

1.  Copy the `.env.example` file into a new `.env.local` file:

    ```bash
    cp .env.example .env.local
    ```

2.  Open the `.env.local` file and add your Web3Auth client ID and Stackup RPC URL:

```bash
WEB3AUTH_CLIENT_ID=Your_Web3Auth_Client_ID
STACKUP_RPC_URL=Your_Stackup_RPC_URL`
```

### Install Dependencies

Install all dependencies by running the following command:

```bash
npm install
```

### Run the Application

You can start the application with the following command:

```bash
npm run dev
```

The application should now be running at [http://localhost:3000](http://localhost:3000/).

## Conclusion

This demonstration app provides a great starting point for integrating self-custodial social authentication into your Ethereum DApp. By utilizing Web3Auth and UserOp.js along with Next.js, it's possible to create a more user-friendly and secure DApp experience.

For further information and help, feel free to raise an issue in this repository.
