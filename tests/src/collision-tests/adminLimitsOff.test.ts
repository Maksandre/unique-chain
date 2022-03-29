// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// This file is part of Unique Network.

// Unique Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Unique Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Unique Network. If not, see <http://www.gnu.org/licenses/>.

/* broken by design
// substrate transactions are sequential, not parallel
// the order of execution is indeterminate

import { IKeyringPair } from '@polkadot/types/types';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import privateKey from '../substrate/privateKey';
import usingApi, { submitTransactionAsync, submitTransactionExpectFailAsync } from '../substrate/substrate-api';
import {
  createCollectionExpectSuccess,
  normalizeAccountId,
  waitNewBlocks,
} from '../util/helpers';

chai.use(chaiAsPromised);
const expect = chai.expect;
let Alice: IKeyringPair;
let Bob: IKeyringPair;
let Ferdie: IKeyringPair;
let Charlie: IKeyringPair;
let Eve: IKeyringPair;
let Dave: IKeyringPair;

before(async () => {
  await usingApi(async () => {
    Alice = privateKey('//Alice');
    Bob = privateKey('//Bob');
    Ferdie = privateKey('//Ferdie');
    Charlie = privateKey('//Charlie');
    Eve = privateKey('//Eve');
    Dave = privateKey('//Dave');
  });
});

describe('Admin limit exceeded collection: ', () => {
  // tslint:disable-next-line: max-line-length
  it('In one block, the owner and admin add new admins to the collection more than the limit ', async () => {
    await usingApi(async (api) => {
      const collectionId = await createCollectionExpectSuccess();

      const chainAdminLimit = (api.consts.unique.collectionAdminsLimit as any).toNumber();
      expect(chainAdminLimit).to.be.equal(5);

      const changeAdminTx1 = api.tx.unique.addCollectionAdmin(collectionId, normalizeAccountId(Eve.address));
      await submitTransactionAsync(Alice, changeAdminTx1);
      const changeAdminTx2 = api.tx.unique.addCollectionAdmin(collectionId, normalizeAccountId(Dave.address));
      await submitTransactionAsync(Alice, changeAdminTx2);
      const changeAdminTx3 = api.tx.unique.addCollectionAdmin(collectionId, normalizeAccountId(Bob.address));
      await submitTransactionAsync(Alice, changeAdminTx3);

      const addAdmOne = api.tx.unique.addCollectionAdmin(collectionId, normalizeAccountId(Ferdie.address));
      const addAdmTwo = api.tx.unique.addCollectionAdmin(collectionId, normalizeAccountId(Charlie.address));
      await Promise.all([
        addAdmOne.signAndSend(Bob),
        addAdmTwo.signAndSend(Alice),
      ]);
      await waitNewBlocks(2);
      const changeAdminTx4 = api.tx.unique.addCollectionAdmin(collectionId, normalizeAccountId(Alice.address));
      await expect(submitTransactionExpectFailAsync(Alice, changeAdminTx4)).to.be.rejected;

      const adminListAfterAddAdmin: any = (await api.query.unique.adminList(collectionId));
      expect(adminListAfterAddAdmin).to.be.contains(normalizeAccountId(Eve.address));
      expect(adminListAfterAddAdmin).to.be.contains(normalizeAccountId(Ferdie.address));
      expect(adminListAfterAddAdmin).not.to.be.contains(normalizeAccountId(Alice.address));
      await waitNewBlocks(2);
    });
  });
});
*/