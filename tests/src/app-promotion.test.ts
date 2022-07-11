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

describe.skip('stake balance extrinsic', () => {
  it('will change balance state to "locked", add it to "staked" map, and increase "totalStaked" amount', async () => {

  });

  it('will throws if stake amount is more than total free balance', async () => {

  });
});

describe.skip('unstake balance', () => {
  it('will change balance state to "reserved", add it to "pendingUnstake" map, and subtract it from totalStaked', async () => {
    // TODO
  });

  it('will return balance to "available" state after the period of unstaking is finished, and subtract it from "pendingUnstake"', async () => {
    // TODO
  });

  it('will throws if unstake amount is more than "staked"', async () => {
    // 
  });

  it('for different accounts in one block is possible', async () => {

  });

  it('will remove from the "staked" map starting from the oldest entry', async () => {
    // alice stakes 1000
    // alice stakes 2000
    // alice stakes 3000
    // expect alice stake is [1000, 2000, 3000]


    // alice unstakes 300
    // expect alice stake is [700, 2000, 3000]
    // expect pending unstake is [300]

    // alice unstakes 1700
    // expect alice stake is [1000, 3000]
    // expect pending unstake is [300, 1700]

    // alice unstakes 4000
    // expect alice stake is []
    // expect pending unstake is [300, 1700, 4000]
  });
});

describe.skip('Admin adress', () => {
  it('can be set by sudo only', async () => {
    // expect failed someAccount.setAdminAddress(substrate address)
    // expect failed admin.setAdminAddress(substrate address)
    // expect success sudo.setAdminAddress(substrate address)
  });

  it('can be unset by sudo', async () => {
    // TODO can?
  });
});

describe.skip('App-promotion collection sponsoring', () => {
  it('can not be set by non admin', async () => {

  });

  it('will set pallet address as confirmed admin for collection without sponsor', async () => {

  });

  it('will set pallet address as confirmed admin for collection with unconfirmed sponsor', async () => {

  });

  it('will set pallet address as confirmed admin for collection with confirmed sponsor', async () => {

  });

  it('will actually sponsor collection transactions in accordance with limits set by collection owner', async () => {
    // 
  });

  it('can be overwritten by collection owner', async () => {

  });

  it('will throw if collection doesn\'t exist', async () => {

  });

  it('will throw if collection was burnt', async () => {

  });
});

describe.skip('app-promotion stopSponsoringCollection', () => {
  it('can be successfully called only by admin', async () => {

  });

  it('will set sponsoring as disabled', async () => {

  });

  it('won\'t affect collection which is not sponsored by pallete', async () => {
    // create collection A
    // set sponsoring for A by a
    // stopSponsoringCollection(A)
    // expect sponsor of A to be a
  });

  it('will throw if collection doesn\'t exist', async () => {
    // expect failed for lastCollectionId + 1000 
    // expect failed for burnt collectiom 
  });
});

describe.skip('app-promotion contract sponsoring', () => {
  it('will set contract sponsoring mode and set palletes address as a sponsor', async () => {

  });

  it('will overwrite sponsoring mode and existed sponsor', async () => {

  });

  it('can be overwritten by contract owner', async () => {

  });

  it('can not be set by non admin', async () => {

  });

  it('what if contract upgradable', async () => {
    // TODO what could be?
  });

  it('will return unused gas fee to app-promotion pallete', async () => {
    // set huge gas fee
  });

  it('will failed for non contract address', async () => {

  });
});

describe.skip('app-promotion stopSponsoringContract', () => {
  // TODO
});

describe.skip('app-promotion rewards', () => {
  // TODO
});
