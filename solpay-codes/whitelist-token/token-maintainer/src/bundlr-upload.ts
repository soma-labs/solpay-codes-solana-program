import {readFileSync} from 'fs';
import Bundlr from '@bundlr-network/client';

(async () => {
    const key = JSON.parse(readFileSync('/home/node/solpay-codes/wallets/whitelist-authority-wallet.json').toString());
    const bundlr = new Bundlr('https://node1.bundlr.network', 'solana', key);
    const currentBalance = (await bundlr.getLoadedBalance()).toNumber();

    console.log(`Current balance: ${currentBalance}`);

    // const tokenLogo = readFileSync(`./files/spaf-token-logo.png`);
    // const tx = bundlr.createTransaction(tokenLogo);

    const metadataFile = readFileSync('./files/wl-token-metadata.json');
    const tx = bundlr.createTransaction(metadataFile);

    const size = tx.size;
    const cost = await bundlr.getPrice(size);

    console.log(
        `Cost to upload: ${cost.toNumber()}`
    );

    // if (currentBalance < cost.toNumber()) {
    //     console.log(`Not enough funds! Funding...`);
    //
    //     const fundStatus = await bundlr.fund(cost);
    //
    //     console.log(
    //         `Funding status:`, fundStatus
    //     );
    //
    //     console.log(
    //         `Balance after funding:`, (await bundlr.getLoadedBalance()).toNumber()
    //     );
    // }
    //
    // await tx.sign();
    // const id = tx.id;
    // const result = await tx.upload();
    //
    // console.log(`Result:`, result);
    // console.log(
    //     `File address: https://arweave.net/${id}`
    // );
})();
