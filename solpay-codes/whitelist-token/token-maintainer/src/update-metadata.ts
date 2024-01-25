import {
    keypairIdentity, Metaplex
} from '@metaplex-foundation/js';
import {clusterApiUrl, Connection, Keypair, PublicKey} from "@solana/web3.js";
import {readFileSync} from "fs";

const TokenMint = '7wXEA2xe5w1iPgvAQsdtRQ6cPFqM4fSrhwtHhEswVzj6';
const TokenMetaDataUri = 'https://arweave.net/VczRzV9OnaFz4N8DIES3-DsaFcw3l3mDFs4nRTderyk';

(async () => {
    const connection = new Connection(clusterApiUrl('mainnet-beta'), 'confirmed');
    const mpl = Metaplex.make(connection);

    mpl.use(
        keypairIdentity(
            Keypair.fromSecretKey(
                Uint8Array.from(
                    JSON.parse(
                        readFileSync('/home/node/solpay-codes/wallets/whitelist-authority-wallet.json').toString()
                    )
                )
            )
        )
    );

    const token = await mpl.nfts().findByMint({mintAddress: new PublicKey(TokenMint)});
    const updateNftOutput = await mpl.nfts().update({
        nftOrSft: token,
        uri: TokenMetaDataUri
    });

    console.log(updateNftOutput);
})();
