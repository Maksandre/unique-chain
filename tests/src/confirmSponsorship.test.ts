//
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.
//

import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { default as usingApi, submitTransactionAsync } from "./substrate/substrate-api";
import { 
  createCollectionExpectSuccess, 
  setCollectionSponsorExpectSuccess, 
  destroyCollectionExpectSuccess, 
  setCollectionSponsorExpectFailure,
  confirmSponsorshipExpectSuccess,
  confirmSponsorshipExpectFailure,
  createItemExpectSuccess,
  findUnusedAddress,
  getGenericResult,
} from "./util/helpers";
import { Keyring } from "@polkadot/api";
import { IKeyringPair } from "@polkadot/types/types";
import type { AccountId } from '@polkadot/types/interfaces';
import { BigNumber } from 'bignumber.js';

chai.use(chaiAsPromised);
const expect = chai.expect;

let alice: IKeyringPair;
let bob: IKeyringPair;
let charlie: IKeyringPair;

describe('integration test: ext. confirmSponsorship():', () => {

  before(async () => {
    await usingApi(async (api) => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri(`//Alice`);
      bob = keyring.addFromUri(`//Bob`);
      charlie = keyring.addFromUri(`//Charlie`);
    });
  });

  it('Confirm collection sponsorship', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await setCollectionSponsorExpectSuccess(collectionId, bob.address);
    await confirmSponsorshipExpectSuccess(collectionId, '//Bob');
  });
  it('Add sponsor to a collection after the same sponsor was already added and confirmed', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await setCollectionSponsorExpectSuccess(collectionId, bob.address);
    await confirmSponsorshipExpectSuccess(collectionId, '//Bob');
    await setCollectionSponsorExpectSuccess(collectionId, bob.address);
  });
  it('Add new sponsor to a collection after another sponsor was already added and confirmed', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await setCollectionSponsorExpectSuccess(collectionId, bob.address);
    await confirmSponsorshipExpectSuccess(collectionId, '//Bob');
    await setCollectionSponsorExpectSuccess(collectionId, charlie.address);
  });

  it.only('Transfer fees are paid by the sponsor after confirmation', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await setCollectionSponsorExpectSuccess(collectionId, bob.address);
    await confirmSponsorshipExpectSuccess(collectionId, '//Bob');
    const itemId = await createItemExpectSuccess(collectionId, 'NFT', '//Alice');

    await usingApi(async (api) => {
      const AsponsorBalance = new BigNumber((await api.query.system.account(bob.address)).toString());

      // Find unused address
      const zeroBalance = await findUnusedAddress(api);
      
      const aliceToZero = api.tx.nft.transfer(zeroBalance.address, collectionId, itemId, 0);
      await submitTransactionAsync(alice, aliceToZero);

      const zeroToAlice = api.tx.nft.transfer(zeroBalance.address, collectionId, itemId, 0);
      const events = await submitTransactionAsync(zeroBalance, zeroToAlice);
      const result = getGenericResult(events);

      const BsponsorBalance = new BigNumber((await api.query.system.account(bob.address)).toString());

      expect(result.success).to.be.true;
      expect(BsponsorBalance.toNumber()).to.be.lessThan(AsponsorBalance.toNumber());
    });

  });

  it.skip('CreateItem fees are paid by the sponsor after confirmation', async () => {
    // const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    // await setCollectionSponsorExpectSuccess(collectionId, bob.address);
    // await confirmSponsorshipExpectSuccess(collectionId, '//Bob');
    expect(false).to.be.true;
  });

});

describe('(!negative test!) integration test: ext. setCollectionSponsor():', () => {
  before(async () => {
    await usingApi(async (api) => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri(`//Alice`);
      bob = keyring.addFromUri(`//Bob`);
      charlie = keyring.addFromUri(`//Charlie`);
    });
  });

  it('(!negative test!) Confirm sponsorship for a collection that never existed', async () => {
    // Find the collection that never existed
    const collectionId = 0;
    await usingApi(async (api) => {
      const collectionId = parseInt((await api.query.nft.createdCollectionCount()).toString()) + 1;
    });

    await confirmSponsorshipExpectFailure(collectionId, '//Bob');
  });

  it('(!negative test!) Confirm sponsorship using a non-sponsor address', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await setCollectionSponsorExpectSuccess(collectionId, bob.address);

    await usingApi(async (api) => {
      const transfer = api.tx.balances.transfer(charlie.address, 1e15);
      await submitTransactionAsync(alice, transfer);
    });

    await confirmSponsorshipExpectFailure(collectionId, '//Charlie');
  });

  it('(!negative test!) Confirm sponsorship using owner address', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await setCollectionSponsorExpectSuccess(collectionId, bob.address);
    await confirmSponsorshipExpectFailure(collectionId, '//Alice');
  });

  it('(!negative test!) Confirm sponsorship without sponsor being set with setCollectionSponsor', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await confirmSponsorshipExpectFailure(collectionId, '//Bob');
  });
    
  it('(!negative test!) Confirm sponsorship in a collection that was destroyed', async () => {
    const collectionId = await createCollectionExpectSuccess('A', 'B', 'C', 'NFT');
    await destroyCollectionExpectSuccess(collectionId);
    await confirmSponsorshipExpectFailure(collectionId, '//Bob');
  });
});
