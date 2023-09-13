const { WsProvider, ApiPromise, Keyring } = require("@polkadot/api");


const main = async () => {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944')
    // const wsProvider = new WsProvider('wss://rpc.polkadot.io')
    const api = await ApiPromise.create({ provider: wsProvider })

    const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

    console.log((await api.query.system.account(ALICE)).toHuman());

    // Construct the keyring after the API (crypto has an async init)
    const keyring = new Keyring({ type: 'sr25519' });

    // Add Alice to our keyring with a hard-derivation path (empty phrase, so uses dev)
    const alice = keyring.addFromUri('//Alice');

    // Create a extrinsic, transferring 12345 units to Bob
    const transfer = api.tx.balances.transfer(BOB, 12345);

    // Sign and send the transaction using our account
    const hash = await transfer.signAndSend(alice);

    console.log('Transfer sent with hash', hash.toHex());
}

main();